use crate::prelude::*;
use console::Key;
use files::slink;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    packages: String,
    config_dir: Vec<String>,
}
impl Config {
    fn new(key: &str) -> Self {
        crate::configs::get(key)
    }
}
pub fn run() {
    // let configs= crate::configs::Configs::new();
    // println!("Got Configs: {configs:#?}");
    let config = Config::new("nvim");
    // println!("nvim config: {config:#?}");
    H1!("NEOVIM and ECOSYSTEM Installation and setup for Linux");

    h2!("Removing old vim");

    crate::linux::manjaro::packages::remove("vim");
    h2!("Installing Neovim and Eco system");
    crate::linux::manjaro::packages::install(&config.packages);

    h2!("Creating directory: {:?}", &config.config_dir[0]);
    exe!("mkdir -pv {}", home_path!(&config.config_dir[0]));

    slink(
        &home_path!(&config.config_dir[1]),
        &home_path!(&config.config_dir[0]),
    );

    h2!("installing plugins");
    exe!("nvim -c 'PlugInstall' -c ':x' -c ':x'");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_nvim_config() {
        crate::configs::init_config();
        println!("Test Config dir path is valid and exist");
        run();

        // assert_eq!(configs.code, PathBuf::from("/path/to/code.toml"));
        // assert_eq!(configs.nvim, PathBuf::from("/path/to/nvim.toml"));
    }
}
