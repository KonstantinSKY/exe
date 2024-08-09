pub use crate::sh::{exec, files};
pub use crossterm::style::Stylize;
pub use std::env;
pub use std::path::{Path, PathBuf};

pub use crate::{cmd, exe, run, h2, home_dir, home_path, H1, warn_print, backup};

pub const WORK_DIR: &str = "Work";
pub const CONFIGS_DIR: &str = "Work/Configs";
pub const BACKUP_DIR: &str = "Work/BackUps";

//configs 
pub const CONFIG_REPO: &str = "https://github.com/KonstantinSKY/Configs.git";


// VS_CODE
pub const CONFIG_FILES: [&str; 3] = ["settings.json", "keybindings.json", ""];

pub const LOCAL_FONT_DIR: &str = ".local/share/fonts";
pub const CONFIG_FONT_DIR: &str = "Configs/fonts";

pub const VSCODE_CONFIG_PATH: &str = ".config/Code - OSS/User/";
pub const VSCODE_CONFIG_SOURCE_PATH: &str = "Work/Configs/vscode/";

pub const VSCODE_EXTENSIONS: [&str; 10] = [
    "vscodevim.vim",
    "asapdotid.manjaro-dark",
    "rust-lang.rust-analyzer",
    "pkief.material-icon-theme",
    "bungcip.better-toml",
    "serayuzgur.crates",
    "streetsidesoftware.code-spell-checker",
    "ms-azuretools.vscode-docker",
    "ms-python.python",
    ""
];


#[must_use]
pub fn get_home_dir() -> String {
    env::var("HOME").unwrap_or_else(|_| String::new())
}
