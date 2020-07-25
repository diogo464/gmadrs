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
    /// The file to list
    file: String,
}

pub fn run(cfg: Config) -> Result<()> {
    let archive = gma::open(cfg.file)?;
    for entry in archive.entries() {
        println!("{}", entry.filename());
    }
    Ok(())
}
