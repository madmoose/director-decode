mod bitmap_data;
mod cast_table;
mod config;
mod file_info;
mod initial_map;
mod key_table;
mod lingo_context;
mod lingo_names;
mod lingo_script;
mod memory_map;
mod script_text;
mod thumbnail;

use std::io::{Result, Seek};

pub use bitmap_data::*;
pub use cast_table::*;
pub use config::*;
pub use file_info::*;
pub use initial_map::*;
pub use key_table::*;
pub use lingo_context::*;
pub use lingo_names::*;
pub use lingo_script::*;
pub use memory_map::*;
pub use script_text::*;
pub use thumbnail::*;

use crate::{
    reader::Reader,
    tags::{Tag, TagAsHex},
};

pub trait Chunk: Sized {
    const TAG: Tag;
    fn read(r: &mut Reader, id: u32) -> Result<Self>;
}

pub fn read_chunk_from_reader<C: Chunk>(reader: &mut Reader, id: u32) -> Result<C> {
    let expected_tag = C::TAG;
    let position = reader.stream_position()?;
    let tag = Tag(reader.read_u32()?);
    let mut size = reader.read_u32()? as u64;

    if position + 8 == reader.stream_len()? && size != 0 {
        eprintln!("Length of final chunk has been mangled.");
        size = 0;
    }

    if tag != expected_tag {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "Expected tag '{}' (size 0x{:x}), found '{}' [{}] at 0x{:x}",
                expected_tag,
                size,
                tag,
                TagAsHex(tag),
                position
            ),
        ));
    }

    let position = reader.stream_position()?;
    let mut chunk_reader = reader.subset(position as usize, size as usize);

    C::read(&mut chunk_reader, id)
}
