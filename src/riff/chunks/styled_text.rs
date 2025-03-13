use std::io::{Read, Result, Seek};

use crate::{
    reader::{ReadBytesExt, Reader},
    riff::tags::{self, Tag},
};

use super::Chunk;

#[allow(unused)]
#[derive(Debug, Default)]
pub struct StyledText {
    text: Vec<u8>,
    style_runs: Vec<StyleRun>,
}

#[allow(unused)]
#[derive(Debug, Default)]
pub struct StyleRun {
    start_offset: i32,
}

impl Chunk for StyledText {
    const TAG: Tag = tags::TAG_STXT;

    fn read(r: &mut Reader, _id: u32) -> Result<Self> {
        let header_size = r.read_be_u32()?;
        let text_size = r.read_be_u32()? as usize;
        let style_size = r.read_be_u32()? as usize;

        assert!(header_size == 12);

        let mut text = vec![0; text_size];
        r.read_exact(&mut text)?;

        let style_run_count = r.read_be_u16()? as usize;
        assert_eq!(style_size, 20 * style_run_count + 2);

        let mut style_runs = Vec::with_capacity(style_run_count);

        for _ in 0..style_run_count {
            let style_run = StyleRun {
                start_offset: r.read_be_i32()?,
            };
            r.seek_relative(16)?;

            style_runs.push(style_run);
        }

        Ok(Self { text, style_runs })
    }
}

impl StyledText {}
