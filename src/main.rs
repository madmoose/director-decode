#![allow(unused)]
#![warn(unused_imports)]
#![feature(io_error_more)]
#![feature(iter_intersperse)]
#![feature(iter_map_windows)]
#![feature(seek_stream_len)]

mod bytes_ext;
mod cast_members;
mod chunks;
mod projector;
mod reader;
mod riff_file;
mod tags;
mod version;

use std::{
    env::args,
    fs::File,
    io::{Read, Seek},
};

use anyhow::Result;
use projector::Projector;
use reader::Reader;
use riff_file::RiffFile;
use tags::*;

fn main() -> Result<()> {
    let filename = args()
        .nth(1)
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::InvalidFilename))?;

    let filename = std::path::Path::new(&filename);

    let mut buf = Vec::new();
    let mut file = File::open(filename)?;
    file.read_to_end(&mut buf)?;
    let mut reader = Reader::new(&buf);

    let mut riff = if filename
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("exe"))
    {
        let mut projector = Projector::read(reader.clone())?;

        projector.display_header();

        let mut riff = projector.read_initial_riff()?;

        println!("initial RIFF:   {}", riff.type_tag());
        println!();

        riff.mmap().display();

        if riff.type_tag() == TAG_APPL {
            if let Some(file) = riff.mmap().first_entry_with_tag(TAG_File) {
                reader.seek(std::io::SeekFrom::Start(file.pos() as u64))?;
                riff = RiffFile::new(reader)?;
            }
        }

        riff.read_key_table()?;
        riff.read_config()?;

        println!(
            "Contained movie RIFF: {} {}\n",
            riff.type_tag(),
            riff.version()
        );

        riff
    } else {
        let mut riff = RiffFile::new(reader)?;

        riff.read_key_table()?;
        riff.read_config()?;

        println!("Movie RIFF: {} {}\n", riff.type_tag(), riff.version());

        riff
    };

    riff.mmap().display();
    if let Some(key_table) = riff.key_table() {
        key_table.display();
    }

    riff.read_file_info()?;
    if let Some(fi) = riff.file_info() {
        fi.display()
    }

    match riff.read_cast_table() {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("No cast table found.");
            return Ok(());
        }
        Err(e) => Err(e),
        Ok(()) => Ok(()),
    }?;

    riff.read_lingo_context()?;
    riff.read_lingo_names()?;
    riff.read_lingo_script()?;

    Ok(())
}
