use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::build_type::BuildSide::Both;
use crate::build_type::DownloadType::All;
use crate::build_type::ManifestType::CurseForge;
use crate::CFManifest;

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
pub struct BuildType<'a> {
    source_manifest: CFManifest,
    download_type: DownloadType,
    build_side: BuildSide,
    manifest_type: ManifestType,
    name: String,
    // {url: path}
    download_list: HashMap<String, &'a Path>,
}

#[derive(Debug)]
pub struct BuildTypeBuilder {
    source_manifest: CFManifest,
    download_type: Option<DownloadType>,
    build_side: Option<BuildSide>,
    manifest_type: Option<ManifestType>,
    name: String,
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

impl BuildType<'_> {
    pub fn builder(manifest: CFManifest, name: &str) -> BuildTypeBuilder {
        BuildTypeBuilder::new(manifest, name)
    }
}

#[allow(dead_code)]
impl BuildTypeBuilder {
    fn new(manifest: CFManifest, name: &str) -> Self {
        Self {
            source_manifest: manifest,
            download_type: None,
            build_side: None,
            manifest_type: None,
            name: name.to_string(),
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

    pub fn build(self) -> BuildType<'static> {
        BuildType {
            source_manifest: self.source_manifest,
            download_type: self.download_type.unwrap_or_default(),
            build_side: self.build_side.unwrap_or_default(),
            manifest_type: self.manifest_type.unwrap_or_default(),
            download_list: HashMap::new(),
            name: self.name,
        }
    }
}
