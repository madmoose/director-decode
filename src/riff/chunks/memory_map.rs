use std::io::Result;

use thousands::Separable;

use crate::{
    reader::Reader,
    riff::tags::{self, Tag},
};

use super::Chunk;

#[allow(unused)]
#[derive(Debug)]
pub struct MemoryMap {
    header_size: u16,
    entry_size: u16,
    chunk_count_max: u32,
    chunk_count_used: u32,
    junk_head: u32,
    junk_head2: u32,
    free_head: u32,
    entries: Vec<MemoryMapEntry>,
}

impl Chunk for MemoryMap {
    const TAG: Tag = tags::TAG_mmap;

    fn read(r: &mut Reader, _id: u32) -> Result<Self> {
        let header_size = r.read_u16()?;
        let entry_size = r.read_u16()?;
        let chunk_count_max = r.read_u32()?;
        let chunk_count_used = r.read_u32()?;
        let junk_head = r.read_u32()?;
        let junk_head2 = r.read_u32()?;
        let free_head = r.read_u32()?;
        let mut entries = Vec::with_capacity(chunk_count_used as usize);

        for id in 0..chunk_count_used {
            entries.push(MemoryMapEntry::read(r, id)?);
        }

        assert!(entries.iter().enumerate().all(|(i, e)| i as u32 == e.id));

        Ok(Self {
            header_size,
            entry_size,
            chunk_count_max,
            chunk_count_used,
            junk_head,
            junk_head2,
            free_head,
            entries,
        })
    }
}

impl MemoryMap {
    pub fn entry_by_index(&self, index: u32) -> Option<&MemoryMapEntry> {
        self.entries.get(index as usize)
    }

    pub fn first_entry_with_tag(&self, tag: Tag) -> Option<&MemoryMapEntry> {
        self.entries.iter().find(|e| e.tag == tag)
    }

    pub fn display(&self) {
        println!("Memory Map:");
        println!("==============================================");
        println!("|      id | tag  |      offset |      length |");
        println!("+---------+------+-------------+-------------+");
        for e in self.entries.iter() {
            println!(
                "|  {:6} | {:4} | {:>11} | {:>11} |",
                e.id,
                e.tag,
                e.pos.separate_with_commas(),
                e.len.separate_with_commas()
            );
        }
        println!("+---------+------+-------------+-------------+");
        println!();
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct MemoryMapEntry {
    id: u32,
    tag: Tag,
    len: u32,
    pos: u32,
    flags: u16,
    unknown0: u16,
    next: u32,
}

impl MemoryMapEntry {
    pub fn read(r: &mut Reader, id: u32) -> Result<Self> {
        Ok(MemoryMapEntry {
            id,
            tag: Tag(r.read_i32()?),
            len: r.read_u32()?,
            pos: r.read_u32()?,
            flags: r.read_u16()?,
            unknown0: r.read_u16()?,
            next: r.read_u32()?,
        })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn tag(&self) -> Tag {
        self.tag
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn pos(&self) -> u32 {
        self.pos
    }
}
