use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};

use crate::{
    bytes_ext::ReadBytesExt,
    reader::Reader,
    tags::{Tag, TAG_VWFI},
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
    const TAG: Tag = TAG_VWFI;

    fn read(r: &mut Reader, id: u32) -> Result<Self> {
        let position = r.stream_position()?;
        let header_size = r.read_be_u32()?;
        let unk0 = r.read_be_u32()?;
        let unk1 = r.read_be_u32()?;
        let flags = r.read_be_u32()?;
        let script_id = (header_size >= 20).then(|| r.read_be_u32()).transpose()?;

        r.seek(SeekFrom::Start(position + header_size as u64))?;

        let entries = r.read_be_u16()?;
        let mut offsets = Vec::with_capacity(entries as usize + 1);
        for i in 0..entries + 1 {
            offsets.push(r.read_be_u32()?);
        }

        let all_increasing = offsets
            .iter()
            .copied()
            .map_windows(|&[ofs0, ofs1]| (ofs0, ofs1))
            .all(|(ofs0, ofs1)| ofs1 >= ofs0);

        if !all_increasing {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid offsets in FileInfo chunks: {:#?}", offsets),
            ));
        }

        let mut changed_by = None;
        let mut created_by = None;
        let mut orig_directory = None;
        let mut preload = None;

        let pos = r.stream_position()? as usize;
        for (i, (pos, len)) in offsets
            .iter()
            .copied()
            .map_windows(|&[ofs0, ofs1]| (pos + ofs0 as usize, (ofs1 - ofs0) as usize))
            .enumerate()
        {
            let mut r2 = r.subset(pos, len);

            if len == 0 {
                continue;
            }

            match i {
                0 => {}
                1 => {
                    changed_by = Some(r2.read_pascal_str()?);
                }
                2 => {
                    created_by = Some(r2.read_pascal_str()?);
                }
                3 => {
                    orig_directory = Some(r2.read_pascal_str()?);
                }
                4 => {
                    preload = Some(r2.read_u16()?);
                }
                _ => {
                    eprintln!("Unhandled part {} in FileInfo", i);
                }
            }
        }

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

        println!("File info:");
        println!("==========");
        println!("Changed by: {}", changed_by);
        println!("Created by: {}", created_by);
        println!("Directory:  {}", orig_directory);
        println!("Preload:    {}", preload);
        println!();
    }
}
