use std::io::Result;

use thousands::Separable;

use crate::{
    reader::Reader,
    riff::tags::{self, Tag},
};

use super::Chunk;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct KeyTable {
    header_size: u16,
    entry_size: u16,
    max_key_count: u32,
    used_key_count: u32,
    entries: Vec<KeyTableEntry>,
}

impl Chunk for KeyTable {
    const TAG: Tag = tags::TAG_KEY_;

    fn read(r: &mut Reader, _id: u32) -> Result<Self> {
        let header_size = r.read_u16()?;
        let entry_size = r.read_u16()?;
        let max_key_count = r.read_u32()?;
        let used_key_count = r.read_u32()?;
        let mut entries = Vec::with_capacity(used_key_count as usize);

        for _ in 0..used_key_count {
            entries.push(KeyTableEntry::read(r)?);
        }

        // TODO(madmoose): Return a proper error
        assert!(entries.is_sorted_by_key(|e| (e.parent, e.tag)));
        assert!(
            entries
                .iter()
                .all(|e| e.id != u32::MAX && e.parent != u32::MAX)
        );

        Ok(KeyTable {
            header_size,
            entry_size,
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
        println!("Key Table:");
        println!("==========");

        println!(
            "header_size:    {}",
            self.header_size.separate_with_commas()
        );
        println!("entry_size:     {}", self.entry_size.separate_with_commas());
        println!(
            "max_key_count:  {}",
            self.max_key_count.separate_with_commas()
        );
        println!(
            "used_key_count: {}",
            self.used_key_count.separate_with_commas()
        );

        println!("====================================");
        println!("|      # |      id | parent | tag  |");
        println!("+--------+---------+--------+------+");
        for (i, e) in self.entries.iter().enumerate() {
            println!("| {:6} |  {:6} | {:6} | {:4} |", i, e.id, e.parent, e.tag);
        }
        println!("+--------+---------+--------+------+");
        println!();
    }

    fn _display_tree(&self, parent: u32, indent: usize) {
        for e in self.chunks_with_parent(parent) {
            for _ in 0..indent {
                print!("\t");
            }
            println!("{}", e.tag);
            self._display_tree(e.id, indent + 1);
        }
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
    id: u32,
    parent: u32,
    tag: Tag,
}

impl KeyTableEntry {
    fn read(r: &mut Reader) -> Result<Self> {
        Ok(KeyTableEntry {
            id: r.read_u32()?,
            parent: r.read_u32()?,
            tag: Tag(r.read_i32()?),
        })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn parent(&self) -> u32 {
        self.parent
    }

    pub fn tag(&self) -> Tag {
        self.tag
    }
}
