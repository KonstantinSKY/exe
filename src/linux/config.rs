use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub work_dirs: Vec<String>,
    pub rc: String,
    pub rc_files:Vec<String>,
    
    pub profile: Vec<String>,
    pub mimeapps_list: Vec<String>,
    pub xresources: Vec<String>,
    
    pub delete_files: String,
    
    pub font_dir: Vec<String>,
    pub font_cache_files: String,
    
    pub trash_dir: String,
    
    pub packages: String,
}

impl Config {
    #[must_use] 
    pub fn new() -> Self {
        crate::configs::get("linux")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}