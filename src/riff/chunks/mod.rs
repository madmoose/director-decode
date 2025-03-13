mod bitmap_data;
mod cast_table;
mod color_lookup_table;
mod config;
mod file_info;
mod frame_labels;
mod initial_map;
mod key_table;
mod lingo_context;
mod lingo_names;
mod lingo_script;
mod memory_map;
mod score;
mod styled_text;
mod thumbnail;

use std::io::{Result, Seek};

pub use bitmap_data::*;
pub use cast_table::*;
pub use color_lookup_table::*;
pub use config::*;
pub use file_info::*;
pub use frame_labels::*;
pub use initial_map::*;
pub use key_table::*;
pub use lingo_context::*;
pub use lingo_names::*;
pub use lingo_script::*;
pub use memory_map::*;
pub use score::*;
pub use styled_text::*;
pub use thumbnail::*;

use crate::reader::Reader;

use super::tags::{Tag, TagAsHex};

pub trait Chunk: Sized {
    const TAG: Tag;
    fn read(r: &mut Reader, id: u32) -> Result<Self>;
}

pub fn read_chunk_from_reader<C: Chunk>(reader: &mut Reader, id: u32) -> Result<C> {
    let expected_tag = C::TAG;
    read_chunk_from_reader_with_tag(reader, id, expected_tag)
}

pub fn read_chunk_from_reader_with_tag<C: Chunk>(
    reader: &mut Reader,
    id: u32,
    expected_tag: Tag,
) -> Result<C> {
    let position = reader.stream_position()?;
    let tag = Tag(reader.read_i32()?);
    let mut size = reader.read_u32()? as u64;

    // TODO: This should be more generous
    if position + 8 == reader.stream_len()? && size != 0 {
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
