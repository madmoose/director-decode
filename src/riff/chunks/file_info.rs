use std::io::Result;

use crate::{
    reader::Reader,
    riff::{
        tags::{self, Tag},
        vlist::VList,
    },
};

use super::Chunk;

#[allow(dead_code)]
#[derive(Debug)]
pub struct FileInfo {
    pub unk0: u32,
    pub unk1: u32,
    pub flags: u32,
    pub script_id: Option<u32>,
    pub changed_by: Option<String>,
    pub created_by: Option<String>,
    pub orig_directory: Option<String>,
    pub preload: Option<u16>,
}

impl Chunk for FileInfo {
    const TAG: Tag = tags::TAG_VWFI;

    fn read(r: &mut Reader, _id: u32) -> Result<Self> {
        let vlist = VList::read_u32(r)?;

        let unk0 = vlist.fixed_number(0).unwrap_or_default();
        let unk1 = vlist.fixed_number(1).unwrap_or_default();
        let flags = vlist.fixed_number(2).unwrap_or_default();
        let script_id = vlist.fixed_number(3);

        let changed_by = vlist.try_get_as_pascal_str(1)?;
        let created_by = vlist.try_get_as_pascal_str(2)?;
        let orig_directory = vlist.try_get_as_pascal_str(3)?;
        let preload = vlist.get(4).map(|mut r| r.read_u16()).transpose()?;

        Ok(FileInfo {
            unk0,
            unk1,
            flags,
            script_id,
            changed_by,
            created_by,
            orig_directory,
            preload,
        })
    }
}

impl FileInfo {
    pub fn display(&self) {
        let changed_by = self.changed_by.as_deref().unwrap_or_default();
        let created_by = self.created_by.as_deref().unwrap_or_default();
        let orig_directory = self.orig_directory.as_deref().unwrap_or_default();
        let preload = self.preload.unwrap_or_default();

        println!("File Info:");
        println!("==========");
        println!("    Changed by: {}", changed_by);
        println!("    Created by: {}", created_by);
        println!("    Directory:  {}", orig_directory);
        println!("    Preload:    {}", preload);
        println!();
    }
}
