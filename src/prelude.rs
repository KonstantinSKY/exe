pub use crossterm::style::Stylize;
pub use std::path::{Path, PathBuf};
pub use std::env;

pub use crate::{cmd, exe, h2, H1, home_path, home_dir};


pub const WORK_DIR: &str = "Work";
pub const CONFIGS_DIR: &str = "Work/Configs";


pub const CONFIG_REPO: &str = "https://github.com/KonstantinSKY/Configs.git";
pub const CONFIG_DIR: &str = "Configs";

pub const LOCAL_FONT_DIR: &str = ".local/share/fonts";
pub const CONFIG_FONT_DIR: &str = "Configs/fonts";


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
pub const MAIN_RC: &str = "Configs/rc";
pub const RC_FILES: [&str; 3] = ["bashrc", "zhsrc", "zshrc"];

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
