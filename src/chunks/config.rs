use std::io::{Result, Seek, SeekFrom};

use crate::{
    bytes_ext::ReadBytesExt,
    reader::Reader,
    tags::{Tag, TAG_VWCF},
};

use super::Chunk;

#[allow(dead_code)]
#[derive(Debug)]
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
}

impl Chunk for Config {
    const TAG: Tag = TAG_VWCF;

    fn read(r: &mut Reader, id: u32) -> Result<Self> {
        let director_version = if r.stream_len()? >= 38 {
            r.seek(SeekFrom::Start(36))?;
            Some(r.read_be_u16()?)
        } else {
            None
        };

        r.seek(SeekFrom::Start(0))?;

        let config = Config {
            len: r.read_be_u16()?,
            file_version: r.read_be_u16()?,
            movie_top: r.read_be_u16()?,
            movie_left: r.read_be_u16()?,
            movie_bottom: r.read_be_u16()?,
            movie_right: r.read_be_u16()?,
            min_member: r.read_be_u16()?,
            max_member: r.read_be_u16()?,
            director_version,
        };

        Ok(config)
    }
}
