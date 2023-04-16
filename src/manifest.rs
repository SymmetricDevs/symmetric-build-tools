use std::collections::HashMap;

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pack {
    version: String,
    author: String,
    name: String,
    minecraft: MinecraftInfo,
    cf: HashMap<String, CFMod>,
    ddl: HashMap<String, DDlMod>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct MinecraftInfo {
    version: String,
    modloader: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CFMod {
    project_id: u64,
    file_id: u64,
    #[serde(default)]
    side: Side,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DDlMod {
    url: String,
    hash: String,
    #[serde(default)]
    side: Side,
} // TODO: Replace hash as a string with something different

#[derive(Debug, Default)]
pub enum Side {
    Server,
    Client,
    #[default]
    Both,
}

impl Serialize for Side {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Side::Server => ser.serialize_str("server"),
            Side::Client => ser.serialize_str("client"),
            Side::Both => ser.serialize_str("both"),
        }
    }
}

impl<'de> Deserialize<'de> for Side {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(SideVisitor)
    }
}

struct SideVisitor;

impl SideVisitor {
    const VARIANTS: &'static [&'static str] = &["server", "client", "both"];
}

impl<'de> Visitor<'de> for SideVisitor {
    type Value = Side;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("variant identifer")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "server" => Ok(Self::Value::Server),
            "client" => Ok(Self::Value::Client),
            "both" => Ok(Self::Value::Both),
            _ => Err(serde::de::Error::unknown_variant(v, SideVisitor::VARIANTS)),
        }
    }
}
