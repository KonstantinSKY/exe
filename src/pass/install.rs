use crate::prelude::*;
use crate::sh::files::slink; 

pub fn run() {
    H1!("Pass Installation and setup for Linux");
    let config = super::config::Config::new("pass");
    
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

    exe!("gpg --import-ownertrust {ot_list_path:?}");

   
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
