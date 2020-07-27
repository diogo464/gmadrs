use super::Result;
use clap::Clap;
use lzma_rs;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

//TODO: add option to remove old file

/// Uncompresses a given .gma file
#[derive(Clap)]
pub struct Config {
    /// The file to decompress
    file: String,
    /// The output file name
    #[clap(short, long)]
    output: Option<String>,
    /// Removes the old file after decompression
    #[clap(long)]
    rm: bool,
}

pub fn run(cfg: Config) -> Result<()> {
    let compressed_file_new_name = cfg.file.clone() + "_compressed";
    std::fs::rename(&cfg.file, &compressed_file_new_name)?;
    let infile = File::open(&compressed_file_new_name)?;
    let mut inreader = BufReader::new(infile);

    let outfile = File::create(cfg.output.unwrap_or(cfg.file))?;
    let mut outwriter = BufWriter::new(outfile);

    lzma_rs::lzma_decompress(&mut inreader, &mut outwriter)?;

    if cfg.rm {
        std::fs::remove_file(compressed_file_new_name)?;
    }

    Ok(())
}
