use super::{Error, Result};
use clap::Clap;
use gma::{AddonTag, AddonType};
use serde::Deserialize;
use std::{convert::TryFrom, fs::File, io::BufWriter, path::PathBuf};
use wildmatch::WildMatch;

impl From<walkdir::Error> for Error {
    fn from(e: walkdir::Error) -> Self {
        if e.io_error().is_some() {
            Self::IOError(e.into_io_error().unwrap())
        } else {
            Self::Custom(Box::new(e))
        }
    }
}

#[derive(Debug, Deserialize)]
struct AddonJsonRaw {
    title: String,
    description: Option<String>,
    author: Option<String>,
    #[serde(rename = "type")]
    ty: String,
    tags: Vec<String>,
    ignore: Vec<String>,
}

struct AddonJson {
    title: String,
    description: Option<String>,
    author: Option<String>,
    ty: AddonType,
    tags: Vec<AddonTag>,
    ignore: Vec<WildMatch>,
}

impl AddonJson {
    fn from_str(string: &str) -> Result<Self> {
        let raw: AddonJsonRaw = serde_json::from_str(&string)
            .map_err(|e| Error::InvalidAddonJson(format!("Invalid addon.json file : {}", e)))?;

        if raw.tags.len() > 2 {
            return Err(Error::ToManyAddonTags);
        }

        let addon_type = AddonType::try_from(raw.ty.as_str())?;
        let addon_tags = {
            let mut tags = Vec::new();
            for t in raw.tags {
                tags.push(AddonTag::try_from(t.as_str())?);
            }
            tags
        };

        let matchers: Vec<WildMatch> = raw.ignore.into_iter().map(|s| WildMatch::new(&s)).collect();

        Ok(Self {
            title: raw.title,
            description: raw.description,
            author: raw.author,
            ty: addon_type,
            tags: addon_tags,
            ignore: matchers,
        })
    }
}

/// Creates a .gma file from a folder
///
/// The folder should have the standard addon folder structure
/// https://wiki.facepunch.com/gmod/Workshop_Addon_Creation
#[derive(Clap)]
pub struct Config {
    /// The directory where addon is
    dir: String,
    /// The file name
    //#[clap(short, long)]
    file: String,
}

pub fn run(cfg: Config) -> Result<()> {
    let dir_path = PathBuf::from(cfg.dir);
    let addon_json = AddonJson::from_str(&std::fs::read_to_string(dir_path.join("addon.json"))?)?;

    let mut builder = gma::GMABuilder::new();
    builder
        .author(addon_json.author.unwrap_or(String::new()))
        .name(addon_json.title)
        .description(addon_json.description.unwrap_or(String::new()))
        .addon_type(addon_json.ty);
    for tag in addon_json.tags {
        builder.addon_tag(tag);
    }

    for entry in walkdir::WalkDir::new(&dir_path) {
        let entry = entry?;
        if entry.file_type().is_dir() {
            continue;
        }

        let filepath = entry.path().to_string_lossy().as_ref().to_owned();
        let ignore_match = addon_json.ignore.iter().find(|m| m.is_match(&filepath));

        if ignore_match.is_none() {
            //read
            builder.file_with_name(
                &filepath,
                entry
                    .path()
                    .strip_prefix(&dir_path)
                    .unwrap()
                    .to_string_lossy()
                    .as_ref(),
            )?;
        } else {
            //ignore
            println!("Ignore file : {}", entry.path().to_string_lossy().as_ref());
        }
    }

    let outfile = File::create(cfg.file)?;
    let mut outfile_writer = BufWriter::new(outfile);

    builder.write_to(&mut outfile_writer)?;

    println!("Done!");
    Ok(())
}
