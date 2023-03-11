use std::collections::HashMap;
use std::path::PathBuf;

use reqwest::header::{self, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::build_type::BuildSide::Both;
use crate::cf_api::{CFResponse, File, Mod};
use crate::{BuildResult, CFManifest};

const CF_BASEURL: &str = "https://api.curseforge.com/v1";

#[derive(Debug, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum DownloadType {
    #[default]
    All,
    NonCf,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub enum BuildSide {
    #[serde(alias = "server")]
    Server,
    #[serde(alias = "client")]
    Client,
    #[serde(alias = "both")]
    #[default]
    Both,
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub enum ManifestType {
    None,
    MultiMC,
    #[default]
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

impl BuildInfo {
    pub fn builder(manifest: CFManifest, name: &str, cf_api_key: String) -> BuildInfoBuilder {
        BuildInfoBuilder::new(manifest, name, cf_api_key)
    }

    pub async fn fetch_mod_downloads(&mut self) -> BuildResult<()> {
        if self.download_type == DownloadType::All {
            let mut headers = HeaderMap::new();
            let key = self.cf_api_key.to_string();
            let mut auth_value = HeaderValue::from_str(&key)?;
            auth_value.set_sensitive(true);
            headers.insert("x-api-key", auth_value);
            headers.insert(header::ACCEPT, HeaderValue::from_str("application/json")?);
            let mut bad_mods: Vec<String> = Vec::new();

            let client = reqwest::Client::builder()
                .default_headers(headers)
                .build()?;

            for file in &self.source_manifest.files {
                if self.build_side == BuildSide::Both
                    || file.side == self.build_side
                    || file.side == Both
                {
                    let file_info: File = client
                        .get(format!(
                            "{}/mods/{}/files/{}",
                            CF_BASEURL, file.project_id, file.file_id
                        ))
                        .send()
                        .await?
                        .json::<CFResponse<_>>()
                        .await?
                        .data;

                    if file_info.download_url.is_empty() {
                        let mod_info: Mod = client
                            .get(format!("{}/mods/{}", CF_BASEURL, file.project_id))
                            .send()
                            .await?
                            .json::<CFResponse<_>>()
                            .await?
                            .data;
                        bad_mods.push(format!(
                            "`{}`: https://www.curseforge.com/minecraft/mc-mods/{}/files/{}",
                            mod_info.name, mod_info.slug, file.file_id
                        ));
                    } else {
                        self.download_list.insert(
                            file_info.download_url.into(),
                            PathBuf::from(format!("out/mod_cache/{}", file_info.file_name)),
                        );
                    }
                }
            }
            //TODO write to file
            println!("Mods that are cringe:");
            println!("{}", bad_mods.join("\n"));
        }
        Ok(())
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
