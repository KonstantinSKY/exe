pub use crate::sh::{exec, files};
pub use crossterm::style::Stylize;
pub use std::env;
pub use std::path::{Path, PathBuf};

pub use crate::{cmd, exe, h2, home_dir, home_path, H1};

pub const WORK_DIR: &str = "Work";
pub const CONFIGS_DIR: &str = "Work/Configs";

pub const CONFIG_REPO: &str = "https://github.com/KonstantinSKY/Configs.git";
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
    "treetsidesoftware.code-spell-checker",
    "ms-azuretools.vscode-docker",
    "",
    "",
];

pub const MAIN_DIRS: [&str; 15] = [
    "Tools",
    "Downloads",
    "Security",
    "Documents",
    "Music",
    "Pictures",
    "Projects",
    "Videos",
    "Configs",
    "Obsidian",
    "BackUps",
    "VirtualBox_VMs",
    "",
    "",
    "",
];
pub const MAIN_RC: &str = "Work/Configs/rc";
pub const RC_FILES: [&str; 3] = ["bashrc", "zhsrc", "zshrc"];

pub const MANJARO_I3_PACKAGES_TO_REMOVE: &str = "bmenu pacui pcmanfm mupdf tesseract";
pub const MANJARO_I3_FILES_TO_DELETE: &str = ".bash_profile";

// const DESKTOP_VARS: [] =[
//         ("GNOME_DESKTOP_SESSION_ID", "GNOME"),
//         ("MATE_DESKTOP_SESSION_ID", "MATE"),
//         ("KDE_FULL_SESSION", "KDE"),
//         ("XDG_CURRENT_DESKTOP", ""),
//         ("DESKTOP_SESSION", ""),
//         ("XDG_SESSION_DESKTOP", ""),
//         ("I3SOCK", "i3"),  // Add detection for i3
//     ];

#[must_use]
pub fn get_home_dir() -> String {
    env::var("HOME").unwrap_or_else(|_| String::new())
}
