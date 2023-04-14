use clap::Parser;
use std::env;
use std::error::Error;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

mod cf_api;

#[derive(Parser, Debug, Default)]
#[command(author = "Symmetric Devs; htmlcsjs", version, about = "Build automation for minecraft modpacks written in rust", long_about = None)]
struct BuildArgs {
    #[cfg(debug_assertions)]
    #[arg(help = "Path of modpack source to use", default_value = "testpack")]
    path: PathBuf,
    #[cfg(not(debug_assertions))]
    #[arg(help = "Path of modpack source to use")]
    path: PathBuf,
}

pub type BuildResult<T> = Result<T, Box<dyn Error + Send + Sync + 'static>>;

#[tokio::main]
async fn main() {
    let args: BuildArgs = BuildArgs::parse();
    let api_key = match env::var("CFAPIKEY") {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to find CFAPIKEY, {}", e);
            String::new()
        }
    };
}
