use std::{
    collections::HashMap,
    io::{ErrorKind, Result, Seek},
};

use crate::{
    bytes_ext::{ByteOrder, ReadBytesExt},
    cast_members::CastMember,
    chunks::{
        read_chunk_from_reader, CastTable, Chunk, Config, FileInfo, InitialMap, KeyTable,
        LingoContext, LingoNames, LingoScript, MemoryMap, ScriptText, Thumbnail,
    },
    reader::Reader,
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
    key_table: Option<KeyTable>,
    config: Option<Config>,
    cast_table: Option<CastTable>,
    cast_members: HashMap<u32, CastMember>,
    lingo_context: Option<LingoContext>,
    lingo_names: Option<LingoNames>,
    lingo_script: Option<LingoScript>,
    file_info: Option<FileInfo>,
    reader: Reader<'a>,
}

impl<'a> RiffFile<'a> {
    pub fn new(reader: Reader<'a>) -> Result<Self> {
        let mut reader = reader;
        let riff_tag = Tag(reader.read_be_u32()?);

        let byte_order = match riff_tag {
            tags::TAG_XFIR => ByteOrder::LittleEndian,
            tags::TAG_RIFX => ByteOrder::BigEndian,
            _ => {
                panic!("Invalid header '{}'", riff_tag);
            }
        };

        reader.set_byte_order(byte_order);

        let size = reader.read_u32()?;
        let type_tag = Tag(reader.read_u32()?);

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
            key_table: None,
            config: None,
            cast_table: None,
            cast_members: HashMap::new(),
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

        Err(std::io::Error::from(std::io::ErrorKind::NotFound))
    }

    fn read_chunk_by_tag<C: Chunk>(&self) -> Result<C> {
        let entry = self
            .mmap()
            .first_entry_with_tag(C::TAG)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;

        let mut reader = self.reader.clone();
        reader.seek(std::io::SeekFrom::Start(entry.pos() as u64))?;

        let chunk = read_chunk_from_reader::<C>(&mut reader, entry.id())?;
        Ok(chunk)
    }

    fn try_read_chunk_by_parent<C: Chunk>(&self, parent: u32) -> Result<Option<C>> {
        let Some(entry) = self
            .key_table()
            .and_then(|key_table| key_table.find_id_of_chunk_with_parent(C::TAG, parent))
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

        Err(std::io::Error::from(std::io::ErrorKind::NotFound))
    }

    pub fn read_key_table(&mut self) -> Result<()> {
        let chunk = self.read_chunk_by_tag()?;
        self.key_table = Some(chunk);

        Ok(())
    }

    pub fn key_table(&self) -> Option<&KeyTable> {
        self.key_table.as_ref()
    }

    pub fn read_config(&mut self) -> Result<()> {
        let config: Config = self.read_chunk_by_parent(GLOBAL_ID)?;

        self.version = Version::new(config.director_version.unwrap_or(config.file_version));

        self.config = Some(config);

        Ok(())
    }

    pub fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }

    pub fn read_cast_table(&mut self) -> Result<()> {
        let cast_table = self.read_chunk_by_parent::<CastTable>(GLOBAL_ID)?;

        for &id in cast_table.cast_member_ids() {
            let mut cast_member = self.read_chunk_by_id::<CastMember>(id)?;

            let Some(key_table) = self.key_table() else {
                continue;
            };

            match &mut cast_member {
                CastMember::Null => {}
                CastMember::Bitmap(bitmap) => {
                    if let Some(bitmap_data) = self.try_read_chunk_by_parent(id)? {
                        let b = cast_member.as_bitmap().unwrap();
                        let filename = format!("image-{:04}.png", id);
                        b.export(&filename, &bitmap_data)?;
                    }
                    if let Some(_thumbnail) = self.try_read_chunk_by_parent::<Thumbnail>(id)? {}
                }
                CastMember::FilmLoop => {}
                CastMember::Text => {}
                CastMember::Palette => {}
                CastMember::Picture => {}
                CastMember::Sound => {}
                CastMember::Button => {}
                CastMember::Shape => {}
                CastMember::Movie => {}
                CastMember::DigitalVideo => {}
                CastMember::Script(script) => {
                    let _script_text = self.try_read_chunk_by_parent::<ScriptText>(id)?;
                }
                CastMember::RTE => {}
            }

            self.cast_members.insert(id, cast_member);
        }

        self.cast_table = Some(cast_table);

        Ok(())
    }

    pub fn cast_table(&self) -> Option<&CastTable> {
        self.cast_table.as_ref()
    }

    fn read_cast_member(&mut self, id: u32) -> Result<()> {
        let cast_member = self.read_chunk_by_id::<CastMember>(id)?;

        self.cast_members.insert(id, cast_member);

        Ok(())
    }

    pub fn cast_member(&self, id: u32) -> Option<&CastMember> {
        self.cast_members.get(&id)
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
