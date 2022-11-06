use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::build_type::BuildSide;

#[derive(Serialize, Deserialize, Debug)]
pub struct CFManifest {
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

impl CFManifest {
    pub fn load_from_file(root: &Path) -> Self {
        let manifest_reader = BufReader::new(
            File::open(root.join("manifest.json")).expect("failed reading manifest file"),
        );
        serde_json::from_reader(manifest_reader).expect("Failed to read JSON")
    }
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
    name: String,
    #[serde(default)]
    side: BuildSide,
}

#[derive(Serialize, Deserialize, Debug)]
struct CFMod {
    #[serde(rename = "projectID")]
    project_id: u64,
    #[serde(rename = "fileID")]
    file_id: u64,
    required: bool,
    #[serde(default)]
    side: BuildSide,
}
