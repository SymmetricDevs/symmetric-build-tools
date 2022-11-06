use crate::build_type::{BuildSide, BuildType, DownloadType};
use clap::Parser;
use manifest::CFManifest;
use std::fmt::Debug;
use std::path::PathBuf;

mod build_type;
mod manifest;

#[derive(Parser, Debug, Default)]
#[command(author = "Symmetric Devs", version, about = "Build automation for minecraft modpacks written in rust", long_about = None)]
struct ProgramInfo {
    #[arg(help = "Path of modpack source to use")]
    path: PathBuf,
}

fn main() {
    let args: ProgramInfo = ProgramInfo::parse();
    let manifest: CFManifest = CFManifest::load_from_file(&args.path);
    let build_type: BuildType = BuildType::builder(manifest, "mmc")
        .build_side(BuildSide::Client)
        .download_type(DownloadType::All)
        .build();
    println!("{:?}", build_type);
}
