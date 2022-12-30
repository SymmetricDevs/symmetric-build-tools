use crate::build_type::{BuildInfo, BuildSide, DownloadType};
use clap::Parser;
use manifest::CFManifest;
use std::env;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

mod build_type;
mod cf_api;
mod manifest;

#[derive(Parser, Debug, Default)]
#[command(author = "Symmetric Devs", version, about = "Build automation for minecraft modpacks written in rust", long_about = None)]
struct ProgramInfo {
    #[cfg(debug_assertions)]
    #[arg(help = "Path of modpack source to use", default_value = "testpack")]
    path: PathBuf,
    #[cfg(not(debug_assertions))]
    #[arg(help = "Path of modpack source to use")]
    path: PathBuf,
}

pub trait DownloadFile {
    fn get_download(&self, info: &BuildInfo) -> Option<(String, &Path)>;
}

fn main() {
    let args: ProgramInfo = ProgramInfo::parse();
    let manifest: CFManifest = CFManifest::load_from_file(&args.path);
    let api_key = match env::var("CFAPIKEY") {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to find CFAPIKEY, {}", e);
            String::new()
        }
    };
    let build_info: BuildInfo = BuildInfo::builder(manifest, "mmc", api_key)
        .build_side(BuildSide::Client)
        .download_type(DownloadType::All)
        .build();

    print!("{:#?}", build_info);
}
