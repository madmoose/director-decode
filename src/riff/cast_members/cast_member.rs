use std::io::{Error, ErrorKind, Result, Seek};

use crate::{
    reader::{ReadBytesExt, Reader},
    riff::{
        cast_members::{BitmapInfo, CastMemberType},
        chunks::Chunk,
        tags::{self, Tag},
        vlist::VList,
    },
};

use super::{Bitmap, Palette, Script, Text};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum CastMember {
    Null,
    Bitmap(Bitmap),
    FilmLoop,
    Text(Text),
    Palette(Palette),
    Picture,
    Sound,
    Button,
    Shape,
    Movie,
    DigitalVideo,
    Script(Script),
    RTE,
}

impl Chunk for CastMember {
    const TAG: Tag = tags::TAG_CASt;

    fn read(r: &mut Reader, id: u32) -> Result<Self> {
        let mut data_len = r.read_be_u16()? as usize;
        let vlist_byte_len = r.read_be_u32()? as usize;

        assert!(data_len > 0);
        let cast_member_type_id = r.read_u8()?;
        data_len -= 1;

        let cast_member_type = CastMemberType::try_from(cast_member_type_id)
            .map_err(|e| Error::new(ErrorKind::InvalidData, format!("{e}")))?;

        let mut _flags = 0;
        if data_len > 1 {
            _flags = r.read_u8()?;
            data_len -= 1;
        }

        let pos = r.stream_position()? as usize;

        let data_reader = r.subset(pos, data_len);
        let mut vlist_reader = r.subset(pos + data_len, vlist_byte_len);

        let vlist = VList::read_u32(&mut vlist_reader)?;
        let name = vlist.try_get_as_pascal_str(1)?;

        let cast_member = match cast_member_type {
            CastMemberType::Null => CastMember::Null,
            CastMemberType::Bitmap => {
                let bitmap_info = BitmapInfo::read(data_reader, id)?;
                CastMember::Bitmap(Bitmap {
                    name,
                    info: bitmap_info,
                    thumbnail: None,
                    data: None,
                })
            }
            CastMemberType::FilmLoop => CastMember::FilmLoop,
            CastMemberType::Text => CastMember::Text(Text::default()),
            CastMemberType::Palette => CastMember::Palette(Palette::default()),
            CastMemberType::Picture => CastMember::Picture,
            CastMemberType::Sound => CastMember::Sound,
            CastMemberType::Button => CastMember::Button,
            CastMemberType::Shape => CastMember::Shape,
            CastMemberType::Movie => CastMember::Movie,
            CastMemberType::DigitalVideo => CastMember::DigitalVideo,
            CastMemberType::Script => CastMember::Script(Script::read(r)?),
            CastMemberType::RTE => CastMember::RTE,
        };

        Ok(cast_member)
    }
}

impl CastMember {
    pub fn cast_member_type(&self) -> CastMemberType {
        match self {
            CastMember::Null => CastMemberType::Null,
            CastMember::Bitmap(_) => CastMemberType::Bitmap,
            CastMember::FilmLoop => CastMemberType::FilmLoop,
            CastMember::Text(_) => CastMemberType::Text,
            CastMember::Palette(_) => CastMemberType::Palette,
            CastMember::Picture => CastMemberType::Picture,
            CastMember::Sound => CastMemberType::Sound,
            CastMember::Button => CastMemberType::Button,
            CastMember::Shape => CastMemberType::Shape,
            CastMember::Movie => CastMemberType::Movie,
            CastMember::DigitalVideo => CastMemberType::DigitalVideo,
            CastMember::Script(_) => CastMemberType::Script,
            CastMember::RTE => CastMemberType::RTE,
        }
    }

    pub fn as_bitmap(&self) -> Option<&Bitmap> {
        match self {
            CastMember::Bitmap(bitmap) => Some(bitmap),
            _ => None,
        }
    }

    pub fn as_palette(&self) -> Option<&Palette> {
        match self {
            CastMember::Palette(palette) => Some(palette),
            _ => None,
        }
    }
}
