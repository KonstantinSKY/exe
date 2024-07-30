use crate::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    packages: String,
    username: String,
    email: String,
    token: String,
}

impl Config {
    fn new(path: &Path) -> Self {
        crate::configs::read_and_parse_toml(path)
    }
}

pub fn run() {
    H1!("GIT Installation and setup for Linux");
    let config_source_path = crate::configs::get_config_path("git");
    let config = Config::new(&config_source_path);

    h2!("Installing GIT and Ecosystem");
    crate::linux::manjaro::packages::update();
    crate::linux::manjaro::packages::install(&config.packages);

    h2!("Checking if git installed");
    exe!("git -v; which git"; true);

    h2!("Show git configs:");
    exe!("git config --list");

    h2!("Setting user name");
    exe!("git config --global user.name {} && git config user.name", config.username);

    h2!("Setting email Your new email address: {}", config.email);
    exe!("git config --global user.email {} && git config user.email", config.email);

    h2!("Setting push behavior");
    exe!("git config --global push.default simple");

    h2!("Setting default branch as main");
    exe!("git config --global init.defaultBranch main");

    h2!("Setting default pull.rebase = false");
    exe!("git config --global pull.rebase false");

    h2!("No addIgnoreFile Advice");
    exe!("git config advice.addIgnoredFile false");

    h2!("Show config:");
    exe!("git config --list"; true);

    h2!("Checking GitHub ssh access");
    exe!("ssh -T git@github.com");

    let config_path = home_path!(CONFIGS_DIR);
    h2!("Change remote type for: {config_path:?}");
    exe!("git -C {config_path:?} remote -v"; true);
    exe!("git -C {config_path:?} remote set-url origin git@github.com:KonstantinSKY/Configs.git");
    exe!("git -C {config_path:?} remote -v"; true);


    H1!("gh setting");
    h2!("Checking gh version");
    exe!("gh --version"; true);

    h2!("GH status:");
    exe!("gh auth status"; true);
    
    h2!("Login in to gh with secret token");
    exe!("echo {} | gh auth login --with-token", config.token);

    h2!("GH status:");
    exe!("gh auth status"; true);
}