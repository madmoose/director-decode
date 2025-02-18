use std::io::{Result, Seek, SeekFrom};

use crate::{bytes_ext::ReadBytesExt, reader::Reader, tags::TAG_Lnam};

use super::Chunk;

#[allow(unused)]
#[derive(Debug)]
pub struct LingoNames {
    unknown0: u32,
    unknown1: u32,
    len1: u32,
    len2: u32,
    names_offset: u16,
    names_count: u16,
    names: Vec<String>,
}

impl Chunk for LingoNames {
    const TAG: crate::tags::Tag = TAG_Lnam;

    fn read(r: &mut Reader, id: u32) -> Result<Self> {
        let unknown0 = r.read_be_u32()?;
        let unknown1 = r.read_be_u32()?;
        let len1 = r.read_be_u32()?;
        let len2 = r.read_be_u32()?;
        let names_offset = r.read_be_u16()?;
        let names_count = r.read_be_u16()?;

        r.seek(SeekFrom::Start(names_offset as u64))?;

        let mut names = Vec::with_capacity(names_count as usize);
        for _ in 0..names_count {
            names.push(r.read_pascal_str()?);
        }

        let lingo_names = LingoNames {
            unknown0,
            unknown1,
            len1,
            len2,
            names_offset,
            names_count,
            names,
        };

        Ok(lingo_names)
    }
}
