#![warn(clippy::pedantic)]  

pub mod prelude;
pub mod styles;
pub mod deploy; 
pub mod rust;
pub mod sh;
pub mod linux;
pub mod code;
pub mod docker;

use clap::Arg;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::fs;


#[must_use] 
pub fn arg_no_confirm() -> Arg {
    Arg::new("noconfirm")
    .help("Skip confirmation flag")
    .short('n')
    .long("noconfirm")
    .action(clap::ArgAction::SetTrue)
}

#[must_use] 
pub fn arg_version() -> Arg {
    Arg::new("version")
    .help("Show versions")
    .short('v')
    .long("version")
    .action(clap::ArgAction::SetTrue)
}

#[derive(Deserialize, Debug)]
pub struct Configs {
    code: PathBuf,
    nvim: PathBuf,
}

impl Configs {
    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string("configs.toml")?;
        let configs: Configs = toml::from_str(&contents)?;
        // configs.paths.validate_paths();
        Ok(configs)
    }
}


pub fn init_config() {
    let configs = Configs::load();
    println!("Configs: {configs:?}");
    // CONFIG.set(config).expect("Failed to set config");
}