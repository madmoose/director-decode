#![allow(unused)]

use std::io::{Error, ErrorKind, Result, Seek};

use crate::reader::{ReadBytesExt, Reader};

pub struct VList<'a> {
    numbers: Vec<u32>,
    entry_count: u16,
    entry_start: u64,
    offsets: Vec<u32>,
    reader: Reader<'a>,
}

enum Kind {
    U16,
    U32,
}

impl<'a> VList<'a> {
    pub fn read_u16(r: &mut Reader<'a>) -> Result<VList<'a>> {
        Self::read(r, Kind::U16)
    }

    pub fn read_u32(r: &mut Reader<'a>) -> Result<VList<'a>> {
        Self::read(r, Kind::U32)
    }

    fn read(r: &mut Reader<'a>, kind: Kind) -> Result<VList<'a>> {
        Self::_read(r, kind)
            .map_err(|err| Error::new(err.kind(), format!("Error reading VList: {}", err)))
    }

    fn _read(r: &mut Reader<'a>, kind: Kind) -> Result<VList<'a>> {
        let offset = r.read_be_u32()?;

        if offset < 4 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid table offset in VList: {}", offset),
            ));
        }

        let item_size = match kind {
            Kind::U16 => 2,
            Kind::U32 => 4,
        };

        if offset % item_size != 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Invalid table offset in VList: {} % {} != 0",
                    offset, item_size
                ),
            ));
        }

        let numbers_count = (offset - 4) / item_size;
        let mut numbers = Vec::with_capacity(numbers_count as usize);
        for _ in 0..numbers_count {
            let item = match kind {
                Kind::U16 => r.read_be_u16()? as u32,
                Kind::U32 => r.read_be_u32()?,
            };
            numbers.push(item);
        }

        let entry_count = r.read_be_u16()?;
        let mut offsets = Vec::with_capacity(entry_count as usize + 1);
        for _ in 0..entry_count + 1 {
            offsets.push(r.read_be_u32()?);
        }

        let all_increasing = offsets
            .iter()
            .copied()
            .map_windows(|&[ofs0, ofs1]| (ofs0, ofs1))
            .all(|(ofs0, ofs1)| ofs1 >= ofs0);

        let entry_start = r.stream_position()?;

        if !all_increasing {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid offset table in VList: {:#?}", offsets),
            ));
        }

        Ok(VList {
            numbers,
            entry_count,
            entry_start,
            offsets,
            reader: r.clone(),
        })
    }

    pub fn fixed_number(&self, index: usize) -> Option<u32> {
        self.numbers.get(index).copied()
    }

    pub fn get(&self, index: usize) -> Option<Reader<'_>> {
        if index + 1 >= self.offsets.len() {
            return None;
        }

        let start = self.entry_start as usize;
        let position = self.offsets[index] as usize;
        let size = self.offsets[index + 1] as usize - position;

        if size == 0 {
            return None;
        }

        Some(self.reader.subset(start + position, size))
    }

    pub fn try_get_as_pascal_str(&self, index: usize) -> Result<Option<String>> {
        self.get(index).map(|mut r| r.read_pascal_str()).transpose()
    }

    pub fn entries(&'a self) -> VListEntries<'a> {
        VListEntries {
            vlist: self,
            index: 0,
        }
    }
}

pub struct VListEntries<'a> {
    vlist: &'a VList<'a>,
    index: usize,
}

impl<'a> Iterator for VListEntries<'a> {
    type Item = VListItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;

        self.vlist.get(index).map(|r| VListItem { r })
    }
}

pub struct VListItem<'a> {
    r: Reader<'a>,
}

impl VListItem<'_> {
    pub fn reader(&self) -> Reader {
        self.r.clone()
    }
}
