use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub packages: String,
    pub security_dir: String,
    pub password_store_link: String,
    pub password_store_source: String,
    pub password_store_repo: String,
    pub ot_list: String,
    // ot_txt: String,
    pub pub_key: String,
    pub sec_key: String,
}

impl Config {
    #[must_use] 
    pub fn new(key: &str) -> Self {
        crate::configs::get(key)
    }
}