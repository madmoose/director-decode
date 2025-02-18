use std::io::Result;

use crate::{
    bytes_ext::ReadBytesExt,
    reader::Reader,
    tags::{Tag, TAG_CAS_},
};

use super::Chunk;

#[allow(dead_code)]
#[derive(Debug)]
pub struct CastTable {
    pub cast_member_ids: Vec<u32>,
}

impl Chunk for CastTable {
    const TAG: Tag = TAG_CAS_;

    fn read(r: &mut Reader, id: u32) -> Result<Self> {
        let mut cast_ids = Vec::new();
        while let Ok(id) = r.read_be_u32() {
            cast_ids.push(id);
        }

        let cast_table = CastTable {
            cast_member_ids: cast_ids,
        };
        Ok(cast_table)
    }
}

impl CastTable {
    pub fn cast_member_ids(&self) -> &[u32] {
        &self.cast_member_ids
    }
}
