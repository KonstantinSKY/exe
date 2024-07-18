use crate::prelude::*;
use serde::Deserialize;
use std::process::{Command, ExitStatus};
use crate::sh::files::slink; 

#[derive(Deserialize, Debug)]
pub struct Config {
    packages: String,
    security_dir: String,
    password_store_link: String,
    password_store_source: String,
    password_store_repo: String,
    ot_list: String,
    ot_txt: String,
    pub_key: String,
    sec_key: String,
}

impl Config {
    fn new(path: &Path) -> Self {
        crate::configs::read_and_parse_toml(path)
    }
}

pub fn run() {
    let config_source_path = crate::configs::get_config_path("pass");
    let config = Config::new(&config_source_path);

    H1!("Pass Installation and setup for Linux");
    
    h2!("Cloning .password-store directory");
    let password_store_source_path = home_path!(&config.password_store_source);
    let password_store_link_path = home_path!(&config.password_store_link);
    exe!(
        "git clone {} {:?}",
        config.password_store_repo,
        password_store_source_path
    );
    slink(&password_store_source_path, &password_store_link_path);


    h2!("Installing ecosystem");
    crate::linux::manjaro::packages::update();
    crate::linux::manjaro::packages::install(&config.packages);

    h2!("Check version");
    exe!("pass version";  true);


    h2!("Importing ownertrust list");
    let ot_list_path = home_path!(&config.security_dir, &config.ot_list); // Update this path as necessary
    if !ot_list_path.exists() {
        println!("{ot_list_path:?} not exists");
        return;
    }
    let sec_key_path = home_path!(&config.security_dir, &config.sec_key);
    if !sec_key_path.exists() {
        println!("{sec_key_path:?} not exists");
        return;
    }
    let pub_key_path = home_path!(&config.security_dir, &config.pub_key);
    if !pub_key_path.exists() {
        println!("{pub_key_path:?} not exists");
        return;
    }

    cmd!("gpg --import-ownertrust {ot_list_path:?}");
    let import_ownertrust_status = Command::new("gpg")
        .arg("--import-ownertrust")
        .arg(format!("{ot_list_path:?}"))
        .status();

    match import_ownertrust_status {
        Ok(status) if status.success() => {
            println!("Ownertrust imported successfully.");
        }
        _ => {
            eprintln!("Failed to import ownertrust.");
        }
    }

   
    // Check for existing GPG keys
    println!("Checking for existing GPG keys...");
    exe!("gpg -k"; true);
    h2!("Importing keys");
    exe!("gpg --import {pub_key_path:?} && gpg --import {sec_key_path:?}");
    println!("Checking for existing GPG keys...");
    exe!("gpg -k"; true);

    h2!("List GnuPG Private,  Public Keys ane trust");
    exe!("gpg --list-keys; gpg --list-secret-keys; gpg --check-trustdb");
    h2!("Checking pass util enabling");
    exe!("pass");
    
}
