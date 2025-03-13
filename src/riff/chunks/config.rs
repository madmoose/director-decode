use std::io::Result;

use crate::{
    reader::{ReadBytesExt, Reader},
    riff::tags::{self, Tag},
};

use super::Chunk;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Config {
    pub len: u16,
    pub file_version: u16,
    pub movie_top: u16,
    pub movie_left: u16,
    pub movie_bottom: u16,
    pub movie_right: u16,
    pub min_member: u16,
    pub max_member: u16,
    pub director_version: Option<u16>,
    pub default_palette_id: Option<i32>,
}

impl Chunk for Config {
    const TAG: Tag = tags::TAG_VWCF;

    fn read(r: &mut Reader, _id: u32) -> Result<Self> {
        let len = r.read_be_u16()?;
        let file_version = r.read_be_u16()?;
        let movie_top = r.read_be_u16()?;
        let movie_left = r.read_be_u16()?;
        let movie_bottom = r.read_be_u16()?;
        let movie_right = r.read_be_u16()?;
        let min_member = r.read_be_u16()?;
        let max_member = r.read_be_u16()?;

        let director_version = r.read_be_u16_at(36).ok();

        let default_palette_id = r
            .read_be_i16_at(0x46)
            .ok()
            .map(|v| v as i32)
            .map(|v| if v <= 0 { v - 1 } else { v });

        let config = Config {
            len,
            file_version,
            movie_top,
            movie_left,
            movie_bottom,
            movie_right,
            min_member,
            max_member,
            director_version,
            default_palette_id,
        };

        Ok(config)
    }
}

impl Config {
    pub fn display(&self) {
        println!("config = {:#?}\n", &self);
    }
}
