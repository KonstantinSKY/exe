use crate::sh::files::slink_pair;

use crate::prelude::*;
use crate::{home_path, linux::os, sh};


use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub config_dirs: Vec<String>,
    pub terminal_files: Vec<String>,
}

impl Config {
    #[must_use] 
    pub fn new() -> Self {
        crate::configs::get("alacritty")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

pub fn install() {
    let config = Config::new();
    os::install("alacritty");
    slink_pair(&config.config_dirs);
    exe!("mkdir -pv .local/bin");
    slink_pair(&config.terminal_files);
    exe!("alacritty &");
}

