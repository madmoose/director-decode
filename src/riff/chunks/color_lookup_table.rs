use std::io::{Result, Seek};

use crate::{
    gfx,
    reader::{ReadBytesExt, Reader},
    riff::tags::{self, Tag},
};

use super::Chunk;

#[derive(Debug)]
pub struct ColorLookupTable {
    pub colors: Vec<gfx::Rgb161616>,
}

impl Chunk for ColorLookupTable {
    const TAG: Tag = tags::TAG_CLUT;

    fn read(r: &mut Reader, _id: u32) -> Result<Self> {
        let size = r.stream_len()? as usize;
        assert!(size % 6 == 0);

        let entries = size / 6;
        assert!(entries <= 256);

        let mut colors = Vec::with_capacity(entries);
        for _ in 0..entries {
            let color = gfx::Rgb161616 {
                r: r.read_be_u16()?,
                g: r.read_be_u16()?,
                b: r.read_be_u16()?,
            };
            colors.push(color);
        }

        Ok(ColorLookupTable { colors })
    }
}
