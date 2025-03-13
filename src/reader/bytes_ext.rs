use std::io::{Error, ErrorKind, Result};

use encoding::Encoding;

#[derive(Copy, Clone, Debug)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[allow(unused)]
pub trait ReadBytesExt: std::io::Read {
    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    #[inline]
    fn read_i8(&mut self) -> Result<i8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }

    fn read_u16(&mut self, byte_order: ByteOrder) -> Result<u16> {
        match byte_order {
            ByteOrder::LittleEndian => self.read_le_u16(),
            ByteOrder::BigEndian => self.read_be_u16(),
        }
    }

    fn read_i16(&mut self, byte_order: ByteOrder) -> Result<i16> {
        match byte_order {
            ByteOrder::LittleEndian => self.read_le_i16(),
            ByteOrder::BigEndian => self.read_be_i16(),
        }
    }

    #[inline]
    fn read_le_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    #[inline]
    fn read_be_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    #[inline]
    fn read_le_i16(&mut self) -> Result<i16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }

    #[inline]
    fn read_be_i16(&mut self) -> Result<i16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }

    fn read_u32(&mut self, byte_order: ByteOrder) -> Result<u32> {
        match byte_order {
            ByteOrder::LittleEndian => self.read_le_u32(),
            ByteOrder::BigEndian => self.read_be_u32(),
        }
    }

    fn read_i32(&mut self, byte_order: ByteOrder) -> Result<i32> {
        match byte_order {
            ByteOrder::LittleEndian => self.read_le_i32(),
            ByteOrder::BigEndian => self.read_be_i32(),
        }
    }

    #[inline]
    fn read_le_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    #[inline]
    fn read_be_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    #[inline]
    fn read_le_i32(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }

    #[inline]
    fn read_be_i32(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    fn read_pascal_str(&mut self) -> Result<String> {
        let len = self.read_u8()?;

        self.read_fixed_str(len as usize)
    }

    fn read_fixed_str(&mut self, len: usize) -> Result<String> {
        let mut bytes = Vec::with_capacity(len);
        let mut bytes_read = 0;
        let mut found_end_of_string = false;

        while bytes_read < len {
            let c = self.read_u8()?;
            bytes_read += 1;

            if found_end_of_string {
            } else if c == 0 {
                found_end_of_string = true;
            } else {
                bytes.push(c);
            }
        }

        let s = encoding::all::ISO_8859_1
            .decode(&bytes, encoding::DecoderTrap::Ignore)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Unable to decode Pascal string"))?;

        Ok(s)
    }
}

impl<R: std::io::Read> ReadBytesExt for R {}
