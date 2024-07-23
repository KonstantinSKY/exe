use crate::prelude::*;
use serde::Deserialize;
use crate::sh::files::slink; 

#[derive(Deserialize, Debug)]
pub struct Config {
    packages: String,
    local_dir: String,
    security_dir: String,
    pub_key_file: String,
    sec_key_file: String,
}

impl Config {
    fn new(path: &Path) -> Self {
        crate::configs::read_and_parse_toml(path)
    }
}

pub fn run() {
    H1!("SSH Installation and setup for Linux");
    
    let config_source_path = crate::configs::get_config_path("ssh");
    let config = Config::new(&config_source_path);

    h2!("Installing OpenSSH");
    crate::linux::manjaro::packages::update();
    crate::linux::manjaro::packages::install(&config.packages);

    h2!("Checking how installed openssh");
    exe!("which ssh"; true);
    exe!("sudo systemctl status sshd --no-pager"; true);

    h2!("Starting and Enabling openssh daemon");
    exe!("sudo systemctl start sshd");
    exe!("sudo systemctl enable sshd");
    exe!("sudo systemctl status sshd --no-pager"; true);


    h2!("Showing IP v4");
    exe!("ip a | grep 'inet '");

    h2!("Creating symlinks for secret and public ssh keys");
    slink(&home_path!(&config.security_dir, &config.pub_key_file), 
    &home_path!(&config.local_dir, &config.pub_key_file));

    slink(&home_path!(&config.security_dir, &config.sec_key_file), 
    &home_path!(&config.local_dir, &config.sec_key_file));

    h2!("Checking GitHub ssh access");
    exe!("ssh -T git@github.com"; true);

    h2!("Checking GitLab ssh access");
    exe!("ssh -T git@gitlab.com"; true);

}