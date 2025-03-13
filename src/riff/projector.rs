use std::io::{Result, Seek, SeekFrom};

use thousands::Separable;

use crate::reader::{ReadBytesExt, Reader};

use super::{
    riff_file::RiffFile,
    tags::{TAG_PJ93, Tag},
};

pub struct Projector<'a> {
    header: Header,
    reader: Reader<'a>,
}

struct Header {
    rifx_ofs: u32,
    fmap_ofs: u32,
    res1_ofs: u32,
    res2_ofs: u32,
    gfx_dll_ofs: u32,
    snd_dll_ofs: u32,
    rifx_ofs_alt: u32,
    flags: u32,
}

impl<'a> Projector<'a> {
    pub fn read(reader: Reader<'a>) -> Result<Self> {
        let mut reader = reader;
        reader.seek(SeekFrom::End(-4))?;

        let offset = reader.read_le_u32()?;
        reader.seek(SeekFrom::Start(offset as u64))?;

        let tag = Tag(reader.read_be_i32()?);
        if tag != TAG_PJ93 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Expected tag PJ93, found '{}'", tag),
            ));
        }

        let header = Header {
            rifx_ofs: reader.read_le_u32()?,
            fmap_ofs: reader.read_le_u32()?,
            res1_ofs: reader.read_le_u32()?,
            res2_ofs: reader.read_le_u32()?,
            gfx_dll_ofs: reader.read_le_u32()?,
            snd_dll_ofs: reader.read_le_u32()?,
            rifx_ofs_alt: reader.read_le_u32()?,
            flags: reader.read_le_u32()?,
        };

        Ok(Self { header, reader })
    }

    pub fn display_header(&self) {
        let h = &self.header;

        let rifx_ofs = h.rifx_ofs.separate_with_commas();
        let fmap_ofs = h.fmap_ofs.separate_with_commas();
        let res1_ofs = h.res1_ofs.separate_with_commas();
        let res2_ofs = h.res2_ofs.separate_with_commas();
        let gfx_dll_ofs = h.gfx_dll_ofs.separate_with_commas();
        let snd_dll_ofs = h.snd_dll_ofs.separate_with_commas();
        let rifx_ofs_alt = h.rifx_ofs_alt.separate_with_commas();
        let flags = h.flags.separate_with_commas();

        println!();
        println!("Projector Header:");
        println!("=================");
        println!("    rifx_ofs:     {:>11}", rifx_ofs);
        println!("    fmap_ofs:     {:>11}", fmap_ofs);
        println!("    res1_ofs:     {:>11}", res1_ofs);
        println!("    res2_ofs:     {:>11}", res2_ofs);
        println!("    gfx_dll_ofs:  {:>11}", gfx_dll_ofs);
        println!("    snd_dll_ofs:  {:>11}", snd_dll_ofs);
        println!("    rifx_ofs_alt: {:>11}", rifx_ofs_alt);
        println!("    flags:        {:>11}", flags);
        println!();
    }

    pub fn read_initial_riff(&mut self) -> Result<RiffFile<'a>> {
        self.reader
            .seek(std::io::SeekFrom::Start(self.header.rifx_ofs as u64))?;

        let riff = RiffFile::new(self.reader.clone())?;

        Ok(riff)
    }
}
