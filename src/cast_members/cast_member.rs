use std::io::{Error, ErrorKind, Result, Seek};

use crate::{
    bytes_ext::ReadBytesExt,
    cast_members::{Bitmap, BitmapInfo, CastMemberType},
    chunks::Chunk,
    reader::Reader,
    tags::{TAG_CASt, Tag},
};

use super::Script;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum CastMember {
    Null,
    Bitmap(Bitmap),
    FilmLoop,
    Text,
    Palette,
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
    const TAG: Tag = TAG_CASt;

    fn read(r: &mut Reader, id: u32) -> Result<Self> {
        let mut data_len = r.read_be_u16()? as usize;
        let info_len = r.read_be_u32()? as usize;

        assert!(data_len > 0);
        let cast_member_type_id = r.read_u8()?;
        data_len -= 1;

        let cast_member_type = CastMemberType::try_from(cast_member_type_id)
            .map_err(|e| Error::new(ErrorKind::InvalidData, format!("{e}")))?;

        let mut flags = 0;
        if data_len > 1 {
            flags = r.read_u8()?;
            data_len -= 1;
        }

        let pos = r.stream_position()? as usize;

        let mut data_reader = r.subset(pos, data_len);
        let mut info_reader = r.subset(pos + data_len, info_len);

        let cast_member = match cast_member_type {
            CastMemberType::Null => CastMember::Null,
            CastMemberType::Bitmap => {
                let bitmap_info = BitmapInfo::read(data_reader, id)?;
                CastMember::Bitmap(Bitmap { info: bitmap_info })
            }
            CastMemberType::FilmLoop => CastMember::FilmLoop,
            CastMemberType::Text => CastMember::Text,
            CastMemberType::Palette => CastMember::Palette,
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
    pub fn as_bitmap(&self) -> Option<&Bitmap> {
        match self {
            CastMember::Bitmap(ref bitmap) => Some(bitmap),
            _ => None,
        }
    }
}
