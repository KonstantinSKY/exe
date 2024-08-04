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

impl Default for Configs {
    fn default() -> Self {
        Self::new()
    }
}

impl Configs {
    #[must_use]
    pub fn new() -> Self {
        let path = home_path!(CONFIGS_DIR, CONFIGS_TOML);
        if let Ok(contents) = fs::read_to_string(&path) {
            // println!("Contents found: {contents}");
            Self::get_from_toml(&contents)
        } else {
            println!("Cant find the main configs.toml file! {path:?}");
            Configs {
                paths: HashMap::new(),
            }
        }
    }

    fn get_from_toml(contents: &str) -> Configs {
        if let Ok(mut configs) = toml::from_str::<Configs>(contents) {
            // println!("Got Configs: {configs:?}");
            configs.canonicalize_paths();
            configs
        } else {
            Configs {
                paths: HashMap::new(),
            }
        }
    }

    fn canonicalize_paths(&mut self) {
        let base_path = home_path!("Work");
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
    pub fn get_config_path(&self, key: &str) -> PathBuf {
        get_config_path(key)
    }
}

#[must_use]
pub fn get<T: for<'de> Deserialize<'de> + std::fmt::Debug>(key: &str) -> T {
    let path = get_config_path(key);
    // read_and_parse_toml(&config_source_path)
    if let Ok(contents) = fs::read_to_string(&path) {
        if let Ok(config) = toml::from_str::<T>(&contents) {
            // println!("Got config: \n {config:#?}");
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

pub fn get_config_path(key: &str) -> PathBuf {
    // println!("CONFIGS FROM path got config: {CONFIGS:#?}");
    if let Some(configs) = CONFIGS.get() {
        // println!("Config got: {configs:?}");
        if let Some(path) = configs.paths.get(key) {
            // println!("{key} path from global config: {path:?}");
            if path.exists() {
                path.clone()
            } else {
                println!("{key} - path does not exist");
                exit(0)
            }
        } else {
            println!("{key} - path is not set");
            exit(0)
        }
    } else {
        println!("Failed to get configs from global!!!");
        exit(0)
    }
}

pub fn init_config() {
    let configs = Configs::new();
    // println!("Main Configs Initialization: {configs:?}");
    match CONFIGS.set(configs) {
        Ok(()) => (),
        Err(_) => println!("Failed to set CONFIGS GLOBAL VARIABLE. It might be already set."),
    }
    // println!("CONFIGS SETT: {CONFIGS:#?}");
}

/// Reads and parses a TOML file into the specified type.
///
/// # Errors
/// This function will exit the program if the file cannot be read or if the
/// contents cannot be parsed as TOML.
// #[must_use]
// pub fn read_and_parse_toml<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
//     if let Ok(contents) = fs::read_to_string(path) {
//         if let Ok(config) = toml::from_str::<T>(&contents) {
//             config
//         } else {
//             println!("Can't convert from TOML file: {path:?}");
//             exit(1);
//         }
//     } else {
//         println!("Can't read file: {path:?}");
//         exit(1);
//     }
// }

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
