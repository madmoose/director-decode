use std::{
    fmt::Debug,
    io::{Cursor, Read, Result, Seek, SeekFrom},
};

use crate::bytes_ext::{ByteOrder, ReadBytesExt};

#[derive(Clone)]
pub struct Reader<'a> {
    inner: Cursor<&'a [u8]>,
    byte_order: ByteOrder,
}

impl Debug for Reader<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reader")
            .field("byte_order", &self.byte_order)
            .finish()
    }
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            inner: Cursor::new(buf),
            byte_order: ByteOrder::LittleEndian,
        }
    }

    pub fn set_byte_order(&mut self, byte_order: ByteOrder) {
        self.byte_order = byte_order;
    }

    pub fn byte_order(&self) -> ByteOrder {
        self.byte_order
    }

    pub fn subset(&self, position: usize, size: usize) -> Self {
        let sub_inner = &self.inner.get_ref()[position..position + size];
        Self {
            inner: Cursor::new(sub_inner),
            byte_order: self.byte_order,
        }
    }

    pub fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        self.inner.read_to_end(buf)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        self.inner.read_u16(self.byte_order)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        self.inner.read_u32(self.byte_order)
    }

    pub fn hex_dump(&mut self) -> Result<()> {
        let mut buffer = Vec::new();

        let pos = self.stream_position()?;

        self.read_to_end(&mut buffer)?;

        for (n, c) in buffer.chunks(16).enumerate() {
            let bs = c
                .iter()
                .map(|b| format!("{b:02x}"))
                .intersperse(" ".to_string())
                .collect::<String>();

            let cs = c
                .iter()
                .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
                .collect::<String>();

            println!("{:08x} {bs:48} {cs}", 16 * n);
        }

        self.seek(SeekFrom::Start(pos))?;

        Ok(())
    }
}

impl Read for Reader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.read(buf)
    }
}

impl Seek for Reader<'_> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.inner.seek(pos)
    }
}
