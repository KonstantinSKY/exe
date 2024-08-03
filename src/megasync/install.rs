
use crate::prelude::*;
use console::Key;
use serde::Deserialize;
use crate::sh::credentials::Credentials;

#[derive(Deserialize, Debug)]
pub struct Config {
    packages: String,
    sync_dirs: Vec<String>,

}

impl Config {
    fn new(key: &str) -> Self {
        crate::configs::get(key)
    }
}


pub fn run() {
    let config = Config::new("megasync");

    H1!("MEGASYNC Daemon Installation and setup for Linux");
    h2!("Installing");
    crate::linux::manjaro::packages::update();
    crate::linux::manjaro::packages::install(&config.packages);
    exe!("mega-version"; true);
    h2!("Login in");

    match  Credentials::input(){
        Ok(credentials) => {
            exe!("mega-login {} {}", credentials.username, credentials.password);

        },
        Err(e) => {
            println!("Error Credentials input: {e}");
            return
        }
    }

    h2!("Login Checking and list of active sync directories");
    exe!("mega-whoami && mega-sync");

    h2!("Sync Security folder");
    for dir_pair in &config.sync_dirs{
        exe!("mega-sync {} && mega-sync", dir_pair);
    }

}