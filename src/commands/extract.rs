use super::Result;
use clap::Clap;
use lzma_rs;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};
/// Uncompresses a given .gma file
#[derive(Clap)]
pub struct Config {
    /// The file to decompress
    file: String,
    /// The output director
    #[clap(short, long)]
    dir: Option<String>,
}

pub fn run(cfg: Config) -> Result<()> {
    let archive = gma::open(&cfg.file)?;

    let outpath = {
        if let Some(dir) = cfg.dir {
            PathBuf::from(dir)
        } else {
            let filepath = Path::new(&cfg.file);
            PathBuf::from(
                filepath
                    .file_stem()
                    .expect("Expected file stem to be valid")
                    .to_owned()
                    .to_str()
                    .expect("This has to work")
                    .to_owned(),
            )
        }
    };
    std::fs::create_dir_all(&outpath)?;

    for entry in archive.entries() {
        let filepath = {
            let mut fp = outpath.clone();
            fp.push(entry.filename());
            fp
        };
        std::fs::create_dir_all(&filepath.parent().expect("It should have a parent"))?;
        let file = File::create(filepath)?;
        let mut writer = BufWriter::new(file);
        archive.read_entry(entry, |_, r| std::io::copy(r, &mut writer))?;
    }

    Ok(())
}
