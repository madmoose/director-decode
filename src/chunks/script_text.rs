use std::io::Result;

use crate::{
    reader::Reader,
    tags::{Tag, TAG_STXT},
};

use super::Chunk;

#[derive(Debug)]
pub struct ScriptText {
    buf: Vec<u8>,
}

impl Chunk for ScriptText {
    const TAG: Tag = TAG_STXT;

    fn read(r: &mut Reader, _id: u32) -> Result<Self> {
        r.hex_dump()?;

        let mut buf = Vec::new();
        r.read_to_end(&mut buf)?;

        Ok(Self { buf })
    }
}

impl ScriptText {
    pub fn buf(&self) -> &[u8] {
        &self.buf
    }
}
