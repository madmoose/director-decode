#![allow(unused, non_upper_case_globals)]

use std::fmt::{Debug, Display};

pub const TAG_APPL: Tag = Tag(0x4150504C); // "APPL"
pub const TAG_BITD: Tag = Tag(0x42495444); // "BITD"
pub const TAG_CAS_: Tag = Tag(0x4341532A); // "CAS*"
pub const TAG_CASt: Tag = Tag(0x43415374); // "CASt"
pub const TAG_CLUT: Tag = Tag(0x434C5554); // "CLUT"
pub const TAG_DRCF: Tag = Tag(0x44524346); // "DRCF"
pub const TAG_File: Tag = Tag(0x46696C65); // "File"
pub const TAG_free: Tag = Tag(0x66726565); // "free"
pub const TAG_imap: Tag = Tag(0x696D6170); // "imap"
pub const TAG_junk: Tag = Tag(0x6A756E6B); // "junk"
pub const TAG_KEY_: Tag = Tag(0x4B45592A); // "KEY*"
pub const TAG_Lctx: Tag = Tag(0x4C637478); // "Lctx"
pub const TAG_Lnam: Tag = Tag(0x4C6E616D); // "Lnam"
pub const TAG_Lscr: Tag = Tag(0x4C736372); // "Lscr"
pub const TAG_mmap: Tag = Tag(0x6D6D6170); // "mmap"
pub const TAG_MV93: Tag = Tag(0x4D563933); // "MV93"
pub const TAG_PJ93: Tag = Tag(0x504A3933); // "PJ93"
pub const TAG_RIFX: Tag = Tag(0x52494648); // "RIFX"
pub const TAG_STXT: Tag = Tag(0x53545854); // "STXT"
pub const TAG_THUM: Tag = Tag(0x5448554D); // "THUM"
pub const TAG_VWCF: Tag = Tag(0x56574346); // "VWCF"
pub const TAG_VWFI: Tag = Tag(0x56574649); // "VWFI"
pub const TAG_VWLB: Tag = Tag(0x56574C42); // "VWLB"
pub const TAG_VWSC: Tag = Tag(0x56575343); // "VWSC"
pub const TAG_XFIR: Tag = Tag(0x58464952); // "XFIR"

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tag(pub i32);

impl From<i32> for Tag {
    fn from(value: i32) -> Self {
        Tag(value)
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        TagAsText(self.0).fmt(f)
    }
}

impl Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = TagAsText(self.0).to_string();
        write!(f, "'{}'", d)
    }
}

pub struct TagAsHex(pub Tag);

impl Display for TagAsHex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut b = self.0.0.to_be_bytes();
        write!(f, "{:02X} {:02X} {:02X} {:02X}", b[0], b[1], b[2], b[3])
    }
}

pub struct TagAsText(pub i32);

impl Display for TagAsText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bytes = self.0.to_be_bytes();

        fn printable(b: u8) -> char {
            if b.is_ascii_graphic() { b as char } else { '.' }
        }

        for b in bytes {
            write!(f, "{}", printable(b))?;
        }

        Ok(())
    }
}
