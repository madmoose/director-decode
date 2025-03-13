use std::io::{Read, Result, Seek};

use crate::{
    gfx,
    reader::{ReadBytesExt, Reader},
    riff::{
        Tempo,
        cast_members::CastMemberId,
        tags::{self, Tag},
    },
};

use super::Chunk;

#[derive(Debug, Default)]
pub struct Score {
    pub frames: Vec<Frame>,
}

impl Chunk for Score {
    const TAG: Tag = tags::TAG_VWSC;

    fn read(r: &mut Reader, _id: u32) -> Result<Self> {
        let length = r.read_be_u32()?;
        let _frames_offset = r.read_be_u32()?;
        let frames_count = r.read_be_u32()?;
        let _frames_version = r.read_be_u16()?;
        let entry_size = r.read_be_u16()?;
        let entry_count = r.read_be_u16()?;
        let _flags = r.read_be_u16()?;

        assert!(entry_size == 20);
        assert!(entry_count >= 2);
        assert!(entry_count <= 50);

        let position = r.stream_position()? as usize;
        let mut r = r.subset(position, length as usize - 20);

        let mut frame_data = [0u8; 20 * 50];
        let mut frame_number = 0;

        let mut frames_data = Vec::<[u8; 1000]>::with_capacity(frames_count as usize);

        while r.stream_remain()? > 0 {
            frame_number += 1;
            Self::decompress_frame(&mut r, &mut frame_data)?;

            frames_data.push(frame_data);
        }

        let mut frames = Vec::with_capacity(frame_number);

        for (frame_number, frame_data) in frames_data.iter().enumerate() {
            let mut frame = Frame {
                index: frame_number as u16,
                ..Frame::default()
            };

            frame
                .sprite_channels
                .reserve_exact(entry_count as usize - 2);

            for (channel_number, data) in frame_data.chunks_exact(20).enumerate() {
                let r = &mut Reader::new(data);

                if channel_number == 0 {
                    r.seek_relative(4)?;
                    let tempo = r.read_i8()?;
                    if tempo != 0 {
                        frame.tempo = Some(Tempo::try_from(tempo).unwrap());
                    }
                }

                // Palette channel
                if channel_number == 1 {
                    let palette_id = r.read_be_i16()?;
                    frame.palette_id = (palette_id != 0).then(|| CastMemberId::new(palette_id));
                    continue;
                }

                if data.iter().all(|&b| b == 0) {
                    continue;
                }

                if channel_number >= 2 {
                    let script_id = r.read_u8()?;
                    let sprite_type = r.read_u8()?;
                    let fore_color = r.read_u8()?;
                    let back_color = r.read_u8()?;
                    let thickness = r.read_u8()?;
                    let ink = r.read_u8()?;

                    let cast_member_id = r.read_be_i16()?;
                    let cast_member_id =
                        (cast_member_id != 0).then(|| CastMemberId::new(cast_member_id));

                    let position = gfx::Pos {
                        y: r.read_be_i16()?,
                        x: r.read_be_i16()?,
                    };
                    let size = gfx::Size {
                        h: r.read_be_i16()?,
                        w: r.read_be_i16()?,
                    };

                    let sprite_channel = SpriteChannel {
                        script_id,
                        sprite_type,
                        fore_color,
                        back_color,
                        thickness,
                        ink,
                        cast_member_id,
                        position,
                        size,
                    };

                    frame
                        .sprite_channels
                        .push((channel_number + 4, sprite_channel));
                }
            }
            frames.push(frame);
        }

        Ok(Score { frames })
    }
}

impl Score {
    fn decompress_frame(r: &mut Reader, frame: &mut [u8; 1000]) -> Result<()> {
        let frame_length = r.read_be_u16()? as usize;

        let mut data_len = frame_length - 2;
        while data_len > 0 {
            let count = r.read_be_u16()? as usize;
            let begin = r.read_be_u16()? as usize;
            data_len -= 4;

            assert!(data_len >= count);

            let end = begin + count;
            assert!(end < frame.len());

            r.read_exact(&mut frame[begin..end])?;
            data_len -= count;
        }

        Ok(())
    }

    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    pub fn get_frame(&self, index: u16) -> Option<&Frame> {
        self.frames.get(index as usize)
    }
}

#[derive(Debug, Default)]
pub struct Frame {
    pub index: u16,
    pub tempo: Option<Tempo>,
    pub palette_id: Option<CastMemberId>,
    pub sprite_channels: Vec<(usize, SpriteChannel)>,
}

impl Frame {
    pub fn display(&self) {
        println!("Frame {}", self.index);
        println!("\tPalette index:    {:?}", self.palette_id);

        for (i, channel) in &self.sprite_channels {
            println!("\tChannel {}", i);
            println!("\t\tScript ID:   {}", channel.script_id);
            println!("\t\tSprite Type: {}", channel.sprite_type);
            println!("\t\tFore Color:  {}", channel.fore_color);
            println!("\t\tBack Color:  {}", channel.back_color);
            println!("\t\tThickness:   {}", channel.thickness);
            println!("\t\tInk:         {:02x}", channel.ink);
            println!("\t\tCast Member: {:?}", channel.cast_member_id);
            println!("\t\tBack Color:  {}", channel.back_color);
            println!("\t\tPosition:    {:?}", channel.position);
            println!("\t\tSize:        {:?}", channel.size);
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct SpriteChannel {
    pub script_id: u8,
    pub sprite_type: u8,
    pub fore_color: u8,
    pub back_color: u8,
    pub thickness: u8,
    pub ink: u8,
    pub cast_member_id: Option<CastMemberId>,
    pub position: gfx::Pos,
    pub size: gfx::Size,
}

impl SpriteChannel {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}
