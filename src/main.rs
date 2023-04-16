use clap::Parser;
use manifest::Pack;
use std::env;
use std::error::Error;
use std::fmt::Debug;
use std::path::PathBuf;

mod cf_api;
mod manifest;

#[derive(Parser, Debug, Default)]
#[command(author = "Symmetric Devs; htmlcsjs", version, about = "Build automation for minecraft modpacks written in rust", long_about = None)]
struct BuildArgs {
    #[cfg(debug_assertions)]
    #[arg(help = "Path of modpack source to use", default_value = "testpack")]
    path: PathBuf,
    #[cfg(not(debug_assertions))]
    #[arg(help = "Path of modpack source to use", default_value = ".")]
    path: PathBuf,
}

pub type BuildResult<T> = Result<T, Box<dyn Error + Send + Sync + 'static>>;

#[tokio::main]
async fn main() {
    let args = {
        let mut a = BuildArgs::parse();
        a.path = a.path.canonicalize().unwrap();
        a
    };
    let api_key = match env::var("CFAPIKEY") {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to find CFAPIKEY, {}", e);
            String::new()
        }
    };
    dbg!(args.path.join("pack.toml"));
    let manifest_str = tokio::fs::read_to_string(args.path.join("pack.toml"))
        .await
        .unwrap();
    let pack_manifest: Pack =
        toml::from_str(&manifest_str).expect("Error parsing Pack manifest file");
    dbg!(pack_manifest);
}
