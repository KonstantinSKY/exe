use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub work_dirs: Vec<String>,
    pub rc: String,
    pub profile: String,
    pub mimeapps_list: String,
    pub delete_files: String,
    pub rc_files: Vec<String>,
    pub local_font_dir: String,
    pub config_font_dir: String,
    pub font_cache_files: String,

}

impl Config {
    #[must_use] 
    pub fn new(key: &str) -> Self {
        crate::configs::get(key)
    }
}