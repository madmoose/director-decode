use std::io::Result;

use crate::{
    reader::{ReadBytesExt, Reader},
    riff::tags::{self, Tag},
};

use super::Chunk;

#[derive(Debug, Default)]
pub struct FrameLabels {
    labels: Vec<FrameLabel>,
}

#[derive(Debug)]
pub struct FrameLabel {
    number: u16,
    text: String,
}

impl Chunk for FrameLabels {
    const TAG: Tag = tags::TAG_VWLB;

    fn read(r: &mut Reader, _id: u32) -> Result<Self> {
        let label_count = r.read_be_u16()? as usize;
        let mut label_offsets = Vec::with_capacity(label_count + 1);
        for _ in 0..label_count + 1 {
            let frame_number = r.read_be_u16()?;
            let text_offset = r.read_be_u16()?;

            label_offsets.push((frame_number, text_offset));
        }

        let mut labels = Vec::with_capacity(label_count);
        for i in 0..label_count {
            let (number, offset0) = label_offsets[i];
            let (_, offset1) = label_offsets[i + 1];
            assert!(offset1 >= offset0);
            let len = (offset1 - offset0) as usize;
            let text = r.read_fixed_str(len)?;

            labels.push(FrameLabel { number, text });
        }

        Ok(Self { labels })
    }
}

impl FrameLabels {
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    pub fn display(&self) {
        println!("Frame Labels:");
        println!("===========================");
        println!("|    # | text             |");
        println!("+------+------------------+");
        for label in &self.labels {
            println!("| {:>4} | {:16} |", label.number, label.text);
        }
        println!("+------+------------------+");
        println!();
    }
}
