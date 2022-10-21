use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(author = "Symmetric Devs", version, about = "Build automation for minecraft modpacks written in rust", long_about = None)]
struct ProgramInfo {
    #[arg(help = "Path of modpack source to use")]
    path: PathBuf,
}


#[derive(Serialize, Deserialize, Debug)]
struct CFManifest {
    minecraft: MCInfo,
    #[serde(rename = "manifestType")]
    manifest_type: String,
    overrides: String,
    #[serde(rename = "manifestVersion")]
    manifest_version: i32,
    version: String,
    author: String,
    name: String,
    #[serde(rename = "externalDeps")]
    direct_download_mods: Vec<DirectDLMod>,
    files: Vec<CFMod>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MCInfo {
    version: String,
    #[serde(rename = "modLoaders")]
    mod_loaders: Vec<ModLoaderInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModLoaderInfo {
    id: String,
    primary: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct DirectDLMod {
    url: String,
    hash: String,
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct CFMod {
    #[serde(rename = "projectID")]
    project_id: u64,
    #[serde(rename = "fileID")]
    file_id: u64,
    required: bool,
}

fn main() {
    let args: ProgramInfo = ProgramInfo::parse();
    let manifest_reader = BufReader::new(File::open(args.path.join("manifest.json")).expect("failed reading manifest file"));
    let manifest: CFManifest = serde_json::from_reader(manifest_reader).expect("mogus");
    println!("{:?}", manifest);
}
