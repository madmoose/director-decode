use std::io::{Result, Seek};

use crate::{
    reader::Reader,
    riff::tags::{self, Tag},
};

use super::Chunk;

#[derive(Debug)]
pub struct Thumbnail {
    buf: Option<Vec<u8>>,
}

impl Chunk for Thumbnail {
    const TAG: Tag = tags::TAG_THUM;

    fn read(r: &mut Reader, _id: u32) -> Result<Self> {
        let buf = if r.stream_len()? == 0 {
            None
        } else {
            let mut buf = Vec::new();
            r.read_to_end(&mut buf)?;
            Some(buf)
        };

        Ok(Self { buf })
    }
}

impl Thumbnail {
    pub fn buf(&self) -> Option<&[u8]> {
        self.buf.as_deref()
    }
}
