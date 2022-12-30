use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::build_type::BuildSide::Both;
use crate::build_type::DownloadType::All;
use crate::build_type::ManifestType::CurseForge;
use crate::{CFManifest, DownloadFile};

#[derive(Debug)]
#[allow(dead_code)]
pub enum DownloadType {
    All,
    NonCf,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BuildSide {
    #[serde(alias = "server")]
    Server,
    #[serde(alias = "client")]
    Client,
    #[serde(alias = "both")]
    Both,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ManifestType {
    None,
    MultiMC,
    CurseForge,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct BuildInfo {
    source_manifest: CFManifest,
    download_type: DownloadType,
    build_side: BuildSide,
    manifest_type: ManifestType,
    name: String,
    cf_api_key: String,
    // {url: path}
    download_list: HashMap<String, PathBuf>,
}

#[derive(Debug)]
pub struct BuildInfoBuilder {
    source_manifest: CFManifest,
    download_type: Option<DownloadType>,
    build_side: Option<BuildSide>,
    manifest_type: Option<ManifestType>,
    name: String,
    cf_api_key: String,
}

impl Default for DownloadType {
    fn default() -> Self {
        All
    }
}

impl Default for BuildSide {
    fn default() -> Self {
        Both
    }
}

impl Default for ManifestType {
    fn default() -> Self {
        CurseForge
    }
}

impl BuildInfo {
    pub fn builder(manifest: CFManifest, name: &str, cf_api_key: String) -> BuildInfoBuilder {
        BuildInfoBuilder::new(manifest, name, cf_api_key)
    }

    pub fn fetch_mod_downloads(&mut self) {
        for file in &self.source_manifest.files {
            if let Some((url, path)) = file.get_download(self) {
                self.download_list.insert(url, path.to_owned());
            }
        }
    }
}

#[allow(dead_code)]
impl BuildInfoBuilder {
    fn new(manifest: CFManifest, name: &str, cf_api_key: String) -> Self {
        Self {
            source_manifest: manifest,
            download_type: None,
            build_side: None,
            manifest_type: None,
            name: name.to_string(),
            cf_api_key,
        }
    }

    pub fn download_type(mut self, download_type: DownloadType) -> Self {
        self.download_type = Some(download_type);
        self
    }

    pub fn build_side(mut self, build_side: BuildSide) -> Self {
        self.build_side = Some(build_side);
        self
    }

    pub fn manifest_type(mut self, manifest_type: ManifestType) -> Self {
        self.manifest_type = Some(manifest_type);
        self
    }

    pub fn build(self) -> BuildInfo {
        BuildInfo {
            source_manifest: self.source_manifest,
            download_type: self.download_type.unwrap_or_default(),
            build_side: self.build_side.unwrap_or_default(),
            manifest_type: self.manifest_type.unwrap_or_default(),
            download_list: HashMap::new(),
            name: self.name,
            cf_api_key: self.cf_api_key,
        }
    }
}
