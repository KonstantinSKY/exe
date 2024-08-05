use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub work_dirs: Vec<String>,
    pub rc: String,
    
    pub local_profile: String,
    pub target_profile: String,
    pub local_mimeapps_list: String,
    pub target_mimeapps_list: String,
    pub local_xresources: String,
    pub target_xresources: String,
    
    pub delete_files: String,
    pub rc_files: Vec<String>,
    pub local_font_dir: String,
    pub config_font_dir: String,
    pub font_cache_files: String,
    pub trash_dir: String,
    pub packages: String,
}

impl Config {
    #[must_use] 
    pub fn new(key: &str) -> Self {
        crate::configs::get(key)
    }
}

