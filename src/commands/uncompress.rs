use super::Result;
use clap::Clap;
use lzma_rs;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

/// Uncompresses a given .gma file
#[derive(Clap)]
pub struct Config {
    /// The file to decompress
    file: String,
    /// The output file name
    #[clap(short, long)]
    output: Option<String>,
}

pub fn run(cfg: Config) -> Result<()> {
    let infile = File::open(cfg.file.clone())?;
    let mut inreader = BufReader::new(infile);

    let outfile = File::create(cfg.output.unwrap_or(cfg.file.clone() + "_uncompressed"))?;
    let mut outwriter = BufWriter::new(outfile);

    lzma_rs::lzma_decompress(&mut inreader, &mut outwriter)?;

    Ok(())
}
