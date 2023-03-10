use crate::build_type::{BuildInfo, BuildSide, DownloadType};
use clap::Parser;
use manifest::CFManifest;
use std::env;
use std::error::Error;
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

pub type BuildResult<T> = Result<T, Box<dyn Error + Send + Sync + 'static>>;
#[tokio::main]
async fn main() {
    let args: ProgramInfo = ProgramInfo::parse();
    let manifest: CFManifest = CFManifest::load_from_file(&args.path);
    let api_key = match env::var("CFAPIKEY") {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to find CFAPIKEY, {}", e);
            String::new()
        }
    };
    let mut build_info: BuildInfo = BuildInfo::builder(manifest, "mmc", api_key)
        .build_side(BuildSide::Both)
        .download_type(DownloadType::All)
        .build();
    if let Err(e) = build_info.fetch_mod_downloads().await {
        println!("Error while fetching mod URLs: {e}");
        let mut src = e.source();
        while let Some(src_e) = src {
            println!("Caused by: {src_e}");
            src = src_e.source();
        }
    }

    println!("{:#?}", build_info);
}
