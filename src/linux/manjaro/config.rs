use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub local_config_dir: String,
    pub target_config_dir: String,
    pub local_mimeapps_list: String,
    pub target_mimeapps_list: String,
    pub local_qt5_conf: String,
    pub target_qt5_conf: String,
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