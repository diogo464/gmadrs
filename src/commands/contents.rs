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
    /// The gma file
    file: String,
    /// The file to print the contents of
    file_to_output: String,
}

pub fn run(cfg: Config) -> Result<()> {
    let archive = gma::open(&cfg.file)?;
    if let Some(file_entry) = archive
        .entries()
        .find(|e| e.filename() == &cfg.file_to_output)
    {
        archive.read_entry(file_entry, |_, r| -> Result<()> {
            let stdout = std::io::stdout();
            let mut lock = stdout.lock();
            std::io::copy(r, &mut lock)?;
            Ok(())
        })??;
    } else {
        //TODO : this has to be error
        println!("File '{}' not found in archive", cfg.file_to_output);
    }

    Ok(())
}
