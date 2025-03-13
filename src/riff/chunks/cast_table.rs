use std::io::Result;

use crate::{
    reader::{ReadBytesExt, Reader},
    riff::{
        cast_members::CastMemberId,
        tags::{self, Tag},
    },
};

use super::Chunk;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CastTable {
    cast_member_ids: Vec<(i16, u32)>,
}

impl Chunk for CastTable {
    const TAG: Tag = tags::TAG_CAS_;

    fn read(r: &mut Reader, _id: u32) -> Result<Self> {
        let mut cast_ids = Vec::new();
        let mut cast_member_id = 1;
        while let Ok(chunk_id) = r.read_be_u32() {
            if chunk_id == 0 {
                continue;
            }
            cast_ids.push((cast_member_id, chunk_id));

            cast_member_id += 1;
        }

        let cast_table = CastTable {
            cast_member_ids: cast_ids,
        };
        Ok(cast_table)
    }
}

impl CastTable {
    pub fn display(&self) {
        println!("Cast Table:");
        println!("====================");
        println!("|      # |      id |");
        println!("+--------+---------+");
        for c in self.cast_member_ids.iter().copied() {
            println!("| {:6} |  {:6} |", c.0, c.1);
        }
        println!("+--------+---------+");
        println!();
    }

    pub fn cast_member_chunk_id(&self, id: CastMemberId) -> Option<u32> {
        self.cast_member_ids
            .binary_search_by_key(&id.id(), |&(id, _)| id)
            .map(|i| &self.cast_member_ids[i].1)
            .copied()
            .ok()
    }
}
