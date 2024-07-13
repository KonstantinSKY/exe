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

// TODO  pub const I3_CONFIG_FILES [($str, &str); 5] =  

// MANJARO PACKAGES
pub const REQUIRED_PACKAGES_1: [&str; 6] = [
    "trash-cli",
    "manjaro-settings-manager",
    "materia-gtk-theme",
    "update-grub",
    "",
    "",
];

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

pub const MANJARO_I3_PACKAGES_TO_REMOVE: &str = "bmenu pacui pcmanfm mupdf libmupdf tesseract-data-afr tesseract" ;
pub const MANJARO_I3_FILES_TO_DELETE: &str = ".bash_profile";
pub const PAMAC_CONFIG: &str = "/etc/pamac.conf";

pub const GRUB_CONFIG: &str = "/etc/default/grub";
pub const GRUB_MANJARO_THEME: &str = "/usr/share/grub/themes/manjaro/theme.txt";

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
