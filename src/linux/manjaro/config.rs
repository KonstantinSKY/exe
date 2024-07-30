use crate::prelude::*;
use serde::Deserialize;

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
    pub unneeded: String,
    pub requirements: String,
    pub requirements_2: String,
    pub internet: String,
    pub communication: String,
}

#[must_use]
pub fn get(key:&str) -> Config{
    let config_source_path = crate::configs::get_config_path(key);
    Config::new(&config_source_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let config = get("manjaro");

        eprintln!("Config: {config:#?}");

    }
}