pub use crossterm::style::Stylize;

pub use std::env;

pub use crate::{cmd, exe, h2, H1};

pub const WORK_DIR: &str = "Work";
pub const CONFIG_REPO: &str = "https://github.com/KonstantinSKY/Configs.git";

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

pub fn get_home_dir() -> String {
    env::var("HOME").unwrap_or_else(|_| String::new())
}
