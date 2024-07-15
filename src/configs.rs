
use crate::prelude::*;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;


use std::process::exit;
pub const WORK_DIR: &str = "Work";
pub const CONFIGS_DIR: &str = "Work/Configs";
pub const CONFIGS_TOML: &str = "configs.toml";



static CONFIGS: OnceCell<Configs> = OnceCell::new();



#[derive(Deserialize, Debug)]
pub struct Configs {
    paths: HashMap<String, PathBuf>,
}

impl Configs {
    fn new() -> Self {
        let path = home_path!(WORK_DIR, CONFIGS_TOML);
        match fs::read_to_string(path) {
            Ok(contents) => match toml::from_str::<Configs>(&contents) {
                Ok(mut configs) => {
                    configs.canonicalize_paths();
                    configs
                }
                Err(_) => Configs {
                    paths: HashMap::new(),
                },
            },
            Err(_) => Configs {
                paths: HashMap::new(),
            },
        }
    }

    fn canonicalize_paths(&mut self) {
        let base_path = home_path!("Work/Configs");
        self.paths.retain(|_key, path| {
            let absolute_path = base_path.join(&path);

            if absolute_path.exists() {
                *path = absolute_path;
                return true;
            }
            println!("Absolute Path: {absolute_path:?} in configs does not exist");
            false
        });
    }

    #[must_use] 
    pub fn get_config_path(self, key: &str) -> PathBuf {
        get_config_path(key)
    }

}

pub fn get_config_path(key: &str) -> PathBuf {
    if let Some(configs) = CONFIGS.get() {
        if let Some(path) = configs.paths.get(key) {
            println!("{key} path from global config: {path:?}");
            if path.exists() {
                path.clone()
            } else {
                println!("{key} path does not exist");
                exit(0)
            }
        } else {
            println!("{key} path is not set");
            exit(0)
        }
    } else {
        println!("Failed to get configs from global");
        exit(0)
    }
}

pub fn init_config() {
    let configs = Configs::new();
    println!("Configs: {configs:?}");
    let code_config_path=get_config_path("code");
    println!("code_config_path {code_config_path:?}");
    // println!("Configs GEt from global: {code:?}");
    // if !configs.code.exists(){
    //     println!("NOT exists")
}
// CONFIG.set(config).expect("Failed to set config");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_config() {
        init_config();

        // assert_eq!(configs.code, PathBuf::from("/path/to/code.toml"));
        // assert_eq!(configs.nvim, PathBuf::from("/path/to/nvim.toml"));
    }
}
