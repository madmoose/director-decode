use std::{fmt::Debug, io::Result};

use crate::{
    reader::Reader,
    riff::tags::{self, Tag},
};

use super::Chunk;

pub struct BitmapData {
    buf: Vec<u8>,
}

impl Chunk for BitmapData {
    const TAG: Tag = tags::TAG_BITD;

    fn read(r: &mut Reader, _id: u32) -> Result<Self> {
        let mut buf = Vec::new();
        r.read_to_end(&mut buf)?;

        Ok(Self { buf })
    }
}

impl Debug for BitmapData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitmapData").finish()
    }
}

impl BitmapData {
    pub fn buf(&self) -> &[u8] {
        &self.buf
    }
}
