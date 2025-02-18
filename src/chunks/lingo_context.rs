use std::io::{Result, Seek, SeekFrom};

use crate::{
    bytes_ext::ReadBytesExt,
    reader::Reader,
    tags::{TAG_Lctx, Tag},
};

use super::Chunk;

#[allow(dead_code)]
#[derive(Debug)]
pub struct LingoContext {
    unknown0: u32,
    unknown1: u32,
    entry_count: u32,
    entry_count_2: u32,
    entries_offset: u16,
    unknown2: u16,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    names_chunk_id: u32,
    valid_count: u16,
    flags: u16,
    free_pointer: u16,
    entries: Vec<LingoContextEntry>,
}

impl Chunk for LingoContext {
    const TAG: Tag = TAG_Lctx;

    fn read(r: &mut Reader, id: u32) -> Result<Self> {
        let unknown0 = r.read_be_u32()?;
        let unknown1 = r.read_be_u32()?;
        let entry_count = r.read_be_u32()?;
        let entry_count_2 = r.read_be_u32()?;
        let entries_offset = r.read_be_u16()?;
        let unknown2 = r.read_be_u16()?;
        let unknown3 = r.read_be_u32()?;
        let unknown4 = r.read_be_u32()?;
        let unknown5 = r.read_be_u32()?;
        let names_chunk_id = r.read_be_u32()?;
        let valid_count = r.read_be_u16()?;
        let flags = r.read_be_u16()?;
        let free_pointer = r.read_be_u16()?;
        let mut entries = Vec::with_capacity(entry_count as usize);

        r.seek(SeekFrom::Start(entries_offset.into()))?;

        for _ in 0..entry_count {
            entries.push(LingoContextEntry::read(r)?);
        }

        let lctx = LingoContext {
            unknown0,
            unknown1,
            entry_count,
            entry_count_2,
            entries_offset,
            unknown2,
            unknown3,
            unknown4,
            unknown5,
            names_chunk_id,
            valid_count,
            flags,
            free_pointer,
            entries,
        };

        Ok(lctx)
    }
}

impl LingoContext {
    pub fn names_chunk_id(&self) -> u32 {
        self.names_chunk_id
    }

    pub fn entries(&self) -> &[LingoContextEntry] {
        &self.entries
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LingoContextEntry {
    unknown0: u32,
    script_id: u32,
    unknown1: u16,
    unknown2: u16,
}

impl LingoContextEntry {
    fn read(r: &mut Reader) -> Result<Self> {
        let entry = LingoContextEntry {
            unknown0: r.read_be_u32()?,
            script_id: r.read_be_u32()?,
            unknown1: r.read_be_u16()?,
            unknown2: r.read_be_u16()?,
        };

        Ok(entry)
    }

    pub fn script_id(&self) -> Option<u32> {
        if self.script_id == u32::MAX {
            return None;
        }

        Some(self.script_id)
    }
}
