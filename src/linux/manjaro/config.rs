use crate::prelude::*;
use serde::Deserialize;
use crate::sh::files::slink; 

#[derive(Deserialize, Debug)]
pub struct Config {
    pub packages: Packages,
}

impl Config {
    fn new(path: &Path) -> Self {
        crate::configs::read_and_parse_toml(path)
    }
}

#[derive(Deserialize, Debug)]
pub struct Packages {
    pub requirements: Vec<String>,
    pub requirements2: Vec<String>,
    pub internet: Vec<String>,
    pub communication: Vec<String>,
}

pub fn get(key:&str) -> Config{
    let config_source_path = crate::configs::get_config_path(key);
    Config::new(&config_source_path)
}