use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub packages: String,
    pub config_dir: Vec<String>,
    pub config_files:Vec<String>,
    pub extensions: Vec<String>,
}

impl Config {
    #[must_use] 
    pub fn new() -> Self {
        crate::configs::get("code")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}