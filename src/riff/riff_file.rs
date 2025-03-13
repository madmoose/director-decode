use std::io::{Error, ErrorKind, Result, Seek};

use crate::reader::{ByteOrder, ReadBytesExt, Reader};

use super::{
    cast_members::{CastMember, CastMemberId},
    chunks::{
        CastTable, Chunk, Config, FileInfo, FrameLabels, InitialMap, KeyTable, LingoContext,
        LingoNames, LingoScript, MemoryMap, Score, read_chunk_from_reader,
        read_chunk_from_reader_with_tag,
    },
    tags::{self, Tag},
    version::Version,
};

const GLOBAL_ID: u32 = 1024;

#[allow(unused)]
#[derive(Debug)]
pub struct RiffFile<'a> {
    size: u32,
    type_tag: Tag,
    byte_order: ByteOrder,
    version: Version,
    imap: InitialMap,
    mmap: MemoryMap,
    key_table: KeyTable,
    config: Config,
    cast_table: CastTable,
    score: Score,
    frame_labels: FrameLabels,
    lingo_context: Option<LingoContext>,
    lingo_names: Option<LingoNames>,
    lingo_script: Option<LingoScript>,
    file_info: Option<FileInfo>,
    reader: Reader<'a>,
}

impl<'a> RiffFile<'a> {
    pub fn new(reader: Reader<'a>) -> Result<Self> {
        let mut reader = reader;
        let riff_tag = Tag(reader.read_be_i32()?);

        let byte_order = match riff_tag {
            tags::TAG_XFIR => ByteOrder::LittleEndian,
            tags::TAG_RIFX => ByteOrder::BigEndian,
            _ => {
                panic!("Invalid header '{}'", riff_tag);
            }
        };

        reader.set_byte_order(byte_order);

        let size = reader.read_u32()?;
        let type_tag = Tag(reader.read_i32()?);

        let imap = read_chunk_from_reader::<InitialMap>(&mut reader, 1)?;

        reader.seek(std::io::SeekFrom::Start(imap.mmap_offset as u64))?;
        let mmap = read_chunk_from_reader::<MemoryMap>(&mut reader, 2)?;

        let riff = RiffFile {
            size,
            type_tag,
            byte_order,
            version: Version::default(),
            imap,
            mmap,
            key_table: KeyTable::default(),
            config: Config::default(),
            cast_table: CastTable::default(),
            score: Score::default(),
            frame_labels: FrameLabels::default(),
            lingo_context: None,
            lingo_names: None,
            lingo_script: None,
            file_info: None,
            reader,
        };

        Ok(riff)
    }

    pub fn type_tag(&self) -> Tag {
        self.type_tag
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn imap(&self) -> &InitialMap {
        &self.imap
    }

    pub fn mmap(&self) -> &MemoryMap {
        &self.mmap
    }

    pub fn file_info(&self) -> Option<&FileInfo> {
        self.file_info.as_ref()
    }

    pub fn try_read_chunk_by_id<C: Chunk>(&self, id: u32) -> Result<Option<C>> {
        let Some(entry) = self.mmap().entry_by_index(id) else {
            return Ok(None);
        };

        let mut reader = self.reader.clone();
        reader.seek(std::io::SeekFrom::Start(entry.pos() as u64))?;

        let chunk = read_chunk_from_reader::<C>(&mut reader, entry.id())?;
        Ok(Some(chunk))
    }

    pub fn read_chunk_by_id<C: Chunk>(&self, id: u32) -> Result<C> {
        if let Some(chunk) = self.try_read_chunk_by_id(id).transpose() {
            return chunk;
        }

        Err(Error::from(std::io::ErrorKind::NotFound))
    }

    fn read_chunk_by_tag<C: Chunk>(&self) -> Result<C> {
        let entry = self
            .mmap()
            .first_entry_with_tag(C::TAG)
            .ok_or_else(|| Error::from(std::io::ErrorKind::NotFound))?;

        let mut reader = self.reader.clone();
        reader.seek(std::io::SeekFrom::Start(entry.pos() as u64))?;

        let chunk = read_chunk_from_reader::<C>(&mut reader, entry.id())?;
        Ok(chunk)
    }

    fn try_read_chunk_by_parent<C: Chunk>(&self, parent: u32) -> Result<Option<C>> {
        let Some(entry) = self
            .key_table()
            .find_id_of_chunk_with_parent(C::TAG, parent)
            .and_then(|index| self.mmap.entry_by_index(index))
        else {
            return Ok(None);
        };

        let mut reader = self.reader.clone();
        reader.seek(std::io::SeekFrom::Start(entry.pos() as u64))?;

        let chunk = read_chunk_from_reader::<C>(&mut reader, entry.id())?;
        Ok(Some(chunk))
    }

    fn read_chunk_by_parent<C: Chunk>(&self, parent: u32) -> Result<C> {
        if let Some(chunk) = self.try_read_chunk_by_parent(parent).transpose() {
            return chunk;
        }

        Err(Error::from(std::io::ErrorKind::NotFound))
    }

    pub fn read_key_table(&mut self) -> Result<()> {
        self.key_table = self.read_chunk_by_tag()?;

        Ok(())
    }

    pub fn key_table(&self) -> &KeyTable {
        &self.key_table
    }

    pub fn read_config(&mut self) -> Result<()> {
        let mut entry = self
            .key_table()
            .find_id_of_chunk_with_parent(tags::TAG_VWCF, GLOBAL_ID)
            .map(|index| (index, tags::TAG_VWCF));

        if entry.is_none() {
            entry = self
                .key_table()
                .find_id_of_chunk_with_parent(tags::TAG_DRCF, GLOBAL_ID)
                .map(|index| (index, tags::TAG_DRCF));
        }

        let Some((index, tag)) = entry else {
            return Err(Error::from(std::io::ErrorKind::NotFound));
        };

        let Some(entry) = self.mmap.entry_by_index(index) else {
            return Err(Error::from(std::io::ErrorKind::NotFound));
        };

        let mut reader = self.reader.clone();
        reader.seek(std::io::SeekFrom::Start(entry.pos() as u64))?;
        let config = read_chunk_from_reader_with_tag::<Config>(&mut reader, entry.id(), tag)?;

        self.config = config;

        Ok(())
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn read_cast_table(&mut self) -> Result<()> {
        self.cast_table = self.read_chunk_by_parent::<CastTable>(GLOBAL_ID)?;

        Ok(())
    }

    pub fn cast_table(&self) -> &CastTable {
        &self.cast_table
    }

    pub fn load_cast_member(&self, id: CastMemberId) -> Result<CastMember> {
        let chunk_id = self
            .cast_table
            .cast_member_chunk_id(id)
            .ok_or(Error::from(ErrorKind::NotFound))?;

        let mut cast_member = self
            .read_chunk_by_id::<CastMember>(chunk_id)
            .map_err(|err| {
                Error::new(
                    err.kind(),
                    format!("RiffFile::read_cast_table: Failed to read cast member: {err}"),
                )
            })?;

        let parent_id = chunk_id;
        match &mut cast_member {
            CastMember::Null => {}
            CastMember::Bitmap(bitmap) => {
                bitmap.data = self.try_read_chunk_by_parent(parent_id)?;
                bitmap.thumbnail = self.try_read_chunk_by_parent(parent_id)?;
            }
            CastMember::FilmLoop => {}
            CastMember::Text(text) => {
                text.styled_text = self.try_read_chunk_by_parent(parent_id)?;
            }
            CastMember::Palette(palette) => {
                palette.clut = self.try_read_chunk_by_parent(parent_id)?;
            }
            CastMember::Picture => {}
            CastMember::Sound => {}
            CastMember::Button => {}
            CastMember::Shape => {}
            CastMember::Movie => {}
            CastMember::DigitalVideo => {}
            CastMember::Script(_script) => {}
            CastMember::RTE => {}
        }

        Ok(cast_member)
    }

    // pub fn get_palette_by_cast_id(&self, cast_id: CastMemberId) -> Option<&cast_members::Palette> {
    //     let cast_member = self.cast_member(cast_id)?;
    //     let palette = match cast_member {
    //         CastMember::Palette(palette) => palette,
    //         _ => return None,
    //     };
    //     Some(palette)
    // }

    pub fn read_score(&mut self) -> Result<()> {
        self.score = self.read_chunk_by_parent::<Score>(GLOBAL_ID)?;

        Ok(())
    }

    pub fn score(&self) -> &Score {
        &self.score
    }

    pub fn read_frame_labels(&mut self) -> Result<()> {
        self.frame_labels = self.read_chunk_by_parent::<FrameLabels>(GLOBAL_ID)?;

        Ok(())
    }

    pub fn frame_labels(&self) -> &FrameLabels {
        &self.frame_labels
    }

    pub fn read_lingo_context(&mut self) -> Result<()> {
        let chunk = self.read_chunk_by_parent(GLOBAL_ID)?;
        self.lingo_context = Some(chunk);

        Ok(())
    }

    pub fn lingo_context(&self) -> Option<&LingoContext> {
        self.lingo_context.as_ref()
    }

    pub fn read_lingo_names(&mut self) -> Result<()> {
        if let Some(lingo_context) = self.lingo_context.as_ref() {
            let chunk = self.read_chunk_by_id(lingo_context.names_chunk_id());

            if chunk
                .as_ref()
                .is_err_and(|e| e.kind() == ErrorKind::NotFound)
            {
                return Ok(());
            }

            self.lingo_names = Some(chunk?);
        }

        Ok(())
    }

    pub fn lingo_names(&self) -> Option<&LingoNames> {
        self.lingo_names.as_ref()
    }

    pub fn read_lingo_script(&mut self) -> Result<()> {
        let chunk = self.read_chunk_by_parent(GLOBAL_ID);

        if chunk
            .as_ref()
            .is_err_and(|e| e.kind() == ErrorKind::NotFound)
        {
            return Ok(());
        }

        self.lingo_script = Some(chunk?);

        Ok(())
    }

    pub fn read_file_info(&mut self) -> Result<()> {
        let chunk = self.read_chunk_by_parent(GLOBAL_ID);

        if chunk
            .as_ref()
            .is_err_and(|e| e.kind() == ErrorKind::NotFound)
        {
            return Ok(());
        }

        self.file_info = Some(chunk?);

        Ok(())
    }
}
