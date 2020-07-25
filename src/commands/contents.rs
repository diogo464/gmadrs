use super::{Error, Result};
use clap::Clap;

/// Writes the contents of a file inside the archive to stdout
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
        return Err(Error::FileNotInArchive(cfg.file_to_output));
    }
    Ok(())
}
