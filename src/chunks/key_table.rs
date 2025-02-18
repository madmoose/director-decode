use std::io::Result;

use crate::{
    reader::Reader,
    tags::{Tag, TAG_KEY_},
};

use super::Chunk;

#[allow(dead_code)]
#[derive(Debug)]
pub struct KeyTable {
    properties_size: u16,
    key_size: u16,
    max_key_count: u32,
    used_key_count: u32,
    entries: Vec<KeyTableEntry>,
}

impl Chunk for KeyTable {
    const TAG: Tag = TAG_KEY_;

    fn read(r: &mut Reader, id: u32) -> Result<Self> {
        let properties_size = r.read_u16()?;
        let key_size = r.read_u16()?;
        let max_key_count = r.read_u32()?;
        let used_key_count = r.read_u32()?;
        let mut entries = Vec::with_capacity(used_key_count as usize);

        for _ in 0..used_key_count {
            entries.push(KeyTableEntry::read(r)?);
        }

        // TODO(madmoose): Return a proper error
        assert!(entries.is_sorted_by_key(|e| (e.parent, e.tag)));
        assert!(entries
            .iter()
            .all(|e| e.id != u32::MAX && e.parent != u32::MAX));

        Ok(KeyTable {
            properties_size,
            key_size,
            max_key_count,
            used_key_count,
            entries,
        })
    }
}

impl KeyTable {
    pub fn find_id_of_chunk_with_parent(&self, tag: Tag, parent: u32) -> Option<u32> {
        self.entries
            .binary_search_by_key(&(parent, tag), |e| (e.parent, e.tag))
            .ok()
            .map(|r| self.entries[r].id)
    }

    pub fn chunks_with_parent(&self, parent: u32) -> ChunksWithParent {
        let begin = self.entries.partition_point(|e| e.parent < parent);
        let end = self.entries.partition_point(|e| e.parent < parent + 1);

        ChunksWithParent {
            key_table: self,
            index: begin,
            end,
        }
    }

    pub fn display(&self) {
        println!("key table:");
        println!("====================================");
        println!("|      # |      id | parent | tag  |");
        println!("+--------+---------+--------+------+");
        for (i, e) in self.entries.iter().enumerate() {
            println!("| {:6} |  {:6} | {:6} | {:4} |", i, e.id, e.parent, e.tag);
        }
        println!("+--------+---------+--------+------+");
        println!();
    }
}

#[derive(Debug)]
pub struct ChunksWithParent<'a> {
    key_table: &'a KeyTable,
    index: usize,
    end: usize,
}

impl<'a> Iterator for ChunksWithParent<'a> {
    type Item = &'a KeyTableEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index != self.end {
            let entry = &self.key_table.entries[self.index];
            self.index += 1;
            Some(entry)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct KeyTableEntry {
    pub id: u32,
    pub parent: u32,
    pub tag: Tag,
}

impl KeyTableEntry {
    fn read(r: &mut Reader) -> Result<Self> {
        Ok(KeyTableEntry {
            id: r.read_u32()?,
            parent: r.read_u32()?,
            tag: Tag(r.read_u32()?),
        })
    }
}
