use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::build_type::BuildSide;

#[derive(Serialize, Deserialize, Debug)]
pub struct CFManifest {
    pub minecraft: MCInfo,
    #[serde(rename = "manifestType")]
    pub manifest_type: String,
    pub overrides: String,
    #[serde(rename = "manifestVersion")]
    pub manifest_version: i32,
    pub version: String,
    pub author: String,
    pub name: String,
    #[serde(rename = "externalDeps")]
    pub direct_download_mods: Vec<DirectDLMod>,
    pub files: Vec<CFMod>,
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
pub struct MCInfo {
    pub version: String,
    #[serde(rename = "modLoaders")]
    pub mod_loaders: Vec<ModLoaderInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModLoaderInfo {
    id: String,
    primary: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectDLMod {
    pub url: String,
    pub hash: String,
    pub name: String,
    #[serde(default)]
    pub side: BuildSide,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CFMod {
    #[serde(rename = "projectID")]
    pub project_id: u64,
    #[serde(rename = "fileID")]
    pub file_id: u64,
    pub required: bool,
    #[serde(default)]
    pub side: BuildSide,
}
