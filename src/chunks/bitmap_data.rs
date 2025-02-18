use std::io::Result;

use crate::{
    reader::Reader,
    tags::{Tag, TAG_BITD},
};

use super::Chunk;

#[derive(Debug)]
pub struct BitmapData {
    buf: Vec<u8>,
}

impl Chunk for BitmapData {
    const TAG: Tag = TAG_BITD;

    fn read(r: &mut Reader, id: u32) -> Result<Self> {
        let mut buf = Vec::new();
        r.read_to_end(&mut buf)?;

        Ok(Self { buf })
    }
}

impl BitmapData {
    pub fn buf(&self) -> &[u8] {
        &self.buf
    }
}
