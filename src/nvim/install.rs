use std::{fs, process::exit};

use crate::prelude::*;
use files::slink;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    packages: String,
    paths: Paths,
    files: Files,
}

#[derive(Deserialize, Debug)]
pub struct Paths {
    config_dir: PathBuf,
    config_source_dir: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Files {
    init_vim: PathBuf,
    configs_init_vim: PathBuf,
}

impl Config {
    fn new(path: &Path) -> Self {
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(config) = toml::from_str::<Config>(&contents) {
                config
            } else {
                println!("Can't convert from TOML file: {path:?}");
                exit(1);
            }
        } else {
            println!("Can't read file: {path:?}");
            exit(1);
        }
    }
}

pub fn run() {
    // let configs= crate::configs::Configs::new();
    // println!("Got Configs: {configs:#?}");
    let config_source_path = crate::configs::get_config_path("nvim");
    let config = Config::new(&config_source_path);
    // println!("nvim config: {config:#?}");
    H1!("NEOVIM and ECOSYSTEM Installation and setup for Linux");
    
    h2!("Installing Neovim and Eco system");
    crate::linux::manjaro::packages::install(&config.packages);

    h2!("Creating directory: {:?}", &config.paths.config_dir);
    exe!("mkdir -pv {:?}", &config.paths.config_dir);

    let link_path = home_path!(&config.paths.config_dir);
    let source_path = home_path!(&config.paths.config_source_dir);
    
    slink(&source_path, &link_path);

    h2!("installing plugins");
    exe!("nvim -c 'PlugInstall' -c ':x' -c ':x'");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_nvim_config() {
        crate::configs::init_config();
        println!("Test Config dir path is valid and existdddd");
        run();

        // assert_eq!(configs.code, PathBuf::from("/path/to/code.toml"));
        // assert_eq!(configs.nvim, PathBuf::from("/path/to/nvim.toml"));
    }
}