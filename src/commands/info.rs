use super::Result;
use clap::Clap;

/// Prints information about a .gma file
#[derive(Clap)]
pub struct Config {
    /// The file to list
    file: String,
}

pub fn run(cfg: Config) -> Result<()> {
    let archive = gma::open(cfg.file)?;
    println!("Version\t\t\t: {}", archive.version());
    println!("Author steam id\t\t: {}", archive.author_steamid());
    println!("Timestamp\t\t: {}", archive.timestamp());
    println!("Name\t\t\t: {}", archive.name());
    println!("Description :\n{}", archive.description());
    println!("Addon Type\t\t: {:?}", archive.addon_type());
    println!("Addon Type\t\t: {:?}", archive.addon_tags());
    println!("Author name\t\t: {}", archive.author());
    println!("Compressed\t\t: {}", archive.compressed());
    println!("Number of files\t\t: {}", archive.entries().count());
    println!(
        "Total files size\t: {}",
        archive.entries().map(|e| e.size()).sum::<u64>()
    );
    Ok(())
}
