use super::Result;
use clap::Clap;

/// Lists the files in a gma archive
#[derive(Clap)]
pub struct Config {
    /// The file to list
    file: String,
    /// Outputs the file sizes
    #[clap(short, long)]
    size: bool,
}

pub fn run(cfg: Config) -> Result<()> {
    let archive = gma::open(cfg.file)?;
    for entry in archive.entries() {
        if cfg.size {
            print!("{}\t", entry.size());
        }
        println!("{}", entry.filename());
    }
    Ok(())
}
