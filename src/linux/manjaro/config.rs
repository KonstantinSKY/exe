use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub global_config_file: String,
    pub i3_config_dir: Vec<String>,
    pub qt5_conf: Vec<String>,
    pub pamac_conf: String,
    pub grub_config: String,
    pub grub_theme: String,
    pub delete_files: String,
    pub packages: Packages,
}

impl Config {
    #[must_use]
    pub fn new(key:&str) -> Self {
        crate::configs::get(key)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let config = Config::new("manjaro");
        eprintln!("Config: {config:#?}");

    }
}