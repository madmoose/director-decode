use std::io::{Result, Seek, SeekFrom};

use crate::{
    reader::{ReadBytesExt, Reader},
    riff::tags::{self, Tag},
};

use super::Chunk;

#[allow(dead_code)]
#[derive(Debug)]
pub struct LingoScript {
    total_length: u32,
    total_length2: u32,
    header_length: u16,
    script_number: u16,
    unk20: u16,
    parent_number: u16,
    script_flags: u32,
    unk42: u16,
    cast_id: u32,
    factory_name_id: u16,
    handler_vectors_count: u16,
    handler_vectors_offset: u32,
    handler_vectors_size: u32,
    properties_count: u16,
    properties_offset: u32,
    globals_count: u16,
    globals_offset: u32,
    handlers_count: u16,
    handlers_offset: u32,
    literals_count: u16,
    literals_offset: u32,
    literals_data_count: u32,
    literals_data_offset: u32,
}

impl Chunk for LingoScript {
    const TAG: Tag = tags::TAG_Lscr;

    fn read(r: &mut Reader, _id: u32) -> Result<Self> {
        r.seek(SeekFrom::Start(8))?;
        let total_length = r.read_be_u32()?;
        let total_length2 = r.read_be_u32()?;
        let header_length = r.read_be_u16()?;
        let script_number = r.read_be_u16()?;
        let unk20 = r.read_be_u16()?;
        let parent_number = r.read_be_u16()?;

        r.seek(SeekFrom::Start(38))?;
        let script_flags = r.read_be_u32()?;
        let unk42 = r.read_be_u16()?;
        let cast_id = r.read_be_u32()?;
        let factory_name_id = r.read_be_u16()?;
        let handler_vectors_count = r.read_be_u16()?;
        let handler_vectors_offset = r.read_be_u32()?;
        let handler_vectors_size = r.read_be_u32()?;
        let properties_count = r.read_be_u16()?;
        let properties_offset = r.read_be_u32()?;
        let globals_count = r.read_be_u16()?;
        let globals_offset = r.read_be_u32()?;
        let handlers_count = r.read_be_u16()?;
        let handlers_offset = r.read_be_u32()?;
        let literals_count = r.read_be_u16()?;
        let literals_offset = r.read_be_u32()?;
        let literals_data_count = r.read_be_u32()?;
        let literals_data_offset = r.read_be_u32()?;

        let mut property_name_ids = Vec::with_capacity(properties_count as usize);
        r.seek(SeekFrom::Start(properties_offset as u64))?;
        for _ in 0..properties_count {
            property_name_ids.push(r.read_be_u16()?);
        }

        let mut global_name_ids = Vec::with_capacity(globals_count as usize);
        r.seek(SeekFrom::Start(globals_offset as u64))?;
        for _ in 0..globals_count {
            global_name_ids.push(r.read_be_u16()?);
        }

        let mut handlers = Vec::with_capacity(handlers_count as usize);
        r.seek(SeekFrom::Start(handlers_offset as u64))?;
        for _ in 0..handlers_count {
            handlers.push(Handler::read(r));
        }

        let script = LingoScript {
            total_length,
            total_length2,
            header_length,
            script_number,
            unk20,
            parent_number,
            script_flags,
            unk42,
            cast_id,
            factory_name_id,
            handler_vectors_count,
            handler_vectors_offset,
            handler_vectors_size,
            properties_count,
            properties_offset,
            globals_count,
            globals_offset,
            handlers_count,
            handlers_offset,
            literals_count,
            literals_offset,
            literals_data_count,
            literals_data_offset,
        };

        Ok(script)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Handler {
    name_id: u16,
    vector_pos: u16,
    compiled_len: u32,
    compiled_offset: u32,
    argument_count: u16,
    argument_offset: u32,
    locals_count: u16,
    locals_offset: u32,
    globals_count: u16,
    globals_offset: u32,
    unknown1: u32,
    unknown2: u16,
    line_count: u16,
    line_offset: u32,
    bytecode: Vec<u8>,
    argument_name_ids: Vec<u16>,
    local_name_ids: Vec<u16>,
    global_name_ids: Vec<u16>,
}

impl Handler {
    fn read(r: &mut Reader) -> Result<Self> {
        let name_id = r.read_be_u16()?;
        let vector_pos = r.read_be_u16()?;
        let compiled_len = r.read_be_u32()?;
        let compiled_offset = r.read_be_u32()?;
        let argument_count = r.read_be_u16()?;
        let argument_offset = r.read_be_u32()?;
        let locals_count = r.read_be_u16()?;
        let locals_offset = r.read_be_u32()?;
        let globals_count = r.read_be_u16()?;
        let globals_offset = r.read_be_u32()?;
        let unknown1 = r.read_be_u32()?;
        let unknown2 = r.read_be_u16()?;
        let line_count = r.read_be_u16()?;
        let line_offset = r.read_be_u32()?;
        let bytecode = Vec::new();
        let argument_name_ids = Vec::new();
        let local_name_ids = Vec::new();
        let global_name_ids = Vec::new();

        Ok(Handler {
            name_id,
            vector_pos,
            compiled_len,
            compiled_offset,
            argument_count,
            argument_offset,
            locals_count,
            locals_offset,
            globals_count,
            globals_offset,
            unknown1,
            unknown2,
            line_count,
            line_offset,
            bytecode,
            argument_name_ids,
            local_name_ids,
            global_name_ids,
        })
    }
}
