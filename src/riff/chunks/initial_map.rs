use std::io::Result;

use crate::{
    reader::Reader,
    riff::tags::{self, Tag},
};

use super::Chunk;

#[allow(unused)]
#[derive(Debug)]
pub struct InitialMap {
    pub mmap_version: u32,
    pub mmap_offset: u32,
    // pub director_version: u32,
}

impl Chunk for InitialMap {
    const TAG: Tag = tags::TAG_imap;

    fn read(r: &mut Reader, _id: u32) -> Result<Self> {
        Ok(Self {
            mmap_version: r.read_u32()?,
            mmap_offset: r.read_u32()?,
            // director_version: r.read_u32()?,
        })
    }
}

impl InitialMap {}
