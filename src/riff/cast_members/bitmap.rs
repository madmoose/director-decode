use std::io::{Error, ErrorKind, Result, Seek};

use crate::{
    gfx,
    reader::{ReadBytesExt, Reader},
    riff::chunks::{BitmapData, Thumbnail},
};

#[derive(Debug)]
pub struct Bitmap {
    pub name: Option<String>,
    pub info: BitmapInfo,
    pub data: Option<BitmapData>,
    pub thumbnail: Option<Thumbnail>,
}

#[derive(Debug)]
pub struct BitmapInfo {
    pub pitch: u16,
    pub rect: gfx::Rect,
    pub reg: gfx::Pos,
    pub bit_depth: u8,
    pub palette_id: i16,
}

impl BitmapInfo {
    pub fn read(r: Reader, _id: u32) -> Result<Self> {
        let mut r = r;

        let a = r.read_be_u16()?;
        let pitch = a & 0xfff;
        let rect = gfx::Rect {
            y0: r.read_be_i16()?,
            x0: r.read_be_i16()?,
            y1: r.read_be_i16()?,
            x1: r.read_be_i16()?,
        };
        let _ = r.read_be_i16()?;
        let _ = r.read_be_i16()?;
        let _ = r.read_be_i16()?;
        let _ = r.read_be_i16()?;

        let reg = gfx::Pos {
            y: r.read_be_i16()?,
            x: r.read_be_i16()?,
        };

        let _ = r.read_u8().unwrap_or_default();

        let bit_depth = r.read_u8().unwrap_or(1);
        // let _ = r.read_be_i16().unwrap_or_default();
        let palette_id = r.read_be_i16().unwrap_or(1) - 1;

        Ok(BitmapInfo {
            pitch,
            rect,
            reg,
            bit_depth,
            palette_id,
        })
    }

    pub fn width(&self) -> i16 {
        self.rect.width()
    }

    pub fn height(&self) -> i16 {
        self.rect.height()
    }
}

impl Bitmap {
    pub fn image(&self) -> Option<gfx::IndexedImage> {
        let mut image = gfx::IndexedImage::new_owned(
            self.info.rect.width() as usize,
            self.info.rect.height() as usize,
        );

        let buf = self.data.as_ref().unwrap().buf();
        let r = Reader::new(buf);

        if self.info.bit_depth == 8 {
            let Ok(len) = decompress_len(r.clone()) else {
                return None;
            };

            let pitch = self.info.pitch as u32;
            let height = len as u32 / pitch;

            image = gfx::IndexedImage::new_owned(pitch as usize, height as usize);
            let data = image.get_mut_data();

            decompress(r, data).unwrap();
        }

        Some(image)
    }
}

pub fn decompress_len(r: Reader) -> Result<usize> {
    let mut r = r;
    let mut remain = r.stream_len()? as usize;
    let mut output_len = 0;

    while remain != 0 {
        let b = r.read_u8()?;
        remain -= 1;

        if b & 0x80 != 0 {
            let len = 257 - (b as usize);

            if remain < 1 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "decompress_len: Unable to decompress",
                ));
            }

            output_len += len;
            remain -= 1;
            r.seek_relative(1)?;
        } else {
            let len = b as usize + 1;

            if remain < len {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "decompress_len: Unable to decompress",
                ));
            }

            output_len += len;
            remain -= len;
            r.seek_relative(len as i64)?;
        }
    }

    Ok(output_len)
}

pub fn decompress(r: Reader, buf: &mut [u8]) -> Result<()> {
    let mut r = r;
    let mut dst_pos = 0;

    while let Ok(b) = r.read_u8() {
        if b & 0x80 != 0 {
            let len = 257 - (b as usize);

            let v = r.read_u8()?;
            for _ in 0..len {
                buf[dst_pos] = v;
                dst_pos += 1;
            }
        } else {
            let len = b as usize + 1;

            for _ in 0..len {
                buf[dst_pos] = r.read_u8()?;
                dst_pos += 1;
            }
        }
    }

    Ok(())
}
