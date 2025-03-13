use std::{
    fs::File,
    io::{Read, Seek},
};

use clap::Parser;

use anyhow::Result;
use director_decoder::{
    reader::Reader,
    riff::{Projector, RiffFile, tags},
};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long)]
    show_mmap: bool,
    #[clap(long)]
    show_key_table: bool,
    #[clap(long)]
    show_config: bool,
    #[clap(long)]
    show_file_info: bool,
    #[clap(long)]
    show_cast_table: bool,
    #[clap(long)]
    show_frame_labels: bool,
    #[clap(long)]
    show_score: bool,
    filename: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let filename = std::path::Path::new(&cli.filename);

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

        if cli.show_mmap {
            riff.mmap().display();
        }

        if riff.type_tag() == tags::TAG_APPL {
            if let Some(file) = riff.mmap().first_entry_with_tag(tags::TAG_File) {
                reader.seek(std::io::SeekFrom::Start(file.pos() as u64))?;
                riff = RiffFile::new(reader)?;
            }
        }

        println!("key_table");
        riff.read_key_table()?;
        println!("config");
        if let Err(err) = riff.read_config() {
            println!("{err}");
        }

        println!(
            "Contained movie RIFF: {} {}\n",
            riff.type_tag(),
            riff.version()
        );

        riff
    } else {
        let mut riff = RiffFile::new(reader)?;

        println!("key_table");
        riff.read_key_table()?;
        println!("config");
        if let Err(err) = riff.read_config() {
            println!("{err}");
        }

        println!("Movie RIFF: {} {}\n", riff.type_tag(), riff.version());

        riff
    };

    if cli.show_mmap {
        riff.mmap().display();
    }
    if cli.show_key_table {
        riff.key_table().display();
    }
    if cli.show_config {
        riff.config().display();
    }

    println!("file_info");
    riff.read_file_info()?;
    if cli.show_file_info {
        riff.file_info().unwrap().display();
    }

    println!("cast_table");
    riff.read_cast_table()?;
    if cli.show_cast_table {
        riff.cast_table().display();
    }

    println!("frame_labels");
    match riff.read_frame_labels() {
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            println!("No frame labels found");
            Ok(())
        }
        v => v,
    }?;

    if cli.show_frame_labels && !riff.frame_labels().is_empty() {
        riff.frame_labels().display();
    }

    println!("score");
    riff.read_score()?;
    if cli.show_score {
        for frame in riff.score().frames() {
            frame.display();
        }
    }

    Ok(())
}
