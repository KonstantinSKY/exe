use crate::linux::os;
use crate::{
    sh::exec::exe,
    sh::files,
    styles::{h1, h2},
};

const EXTENSIONS: [&str; 10] = [
    "vscodevim.vim",
    "asapdotid.manjaro-dark",
    "rust-lang.rust-analyzer",
    "pkief.material-icon-theme",
    "bungcip.better-toml",
    "serayuzgur.crates",
    "treetsidesoftware.code-spell-checker",
    "",
    "",
    "",
];

const CONFIG_PATH: &str = "~/.config/Code - OSS/User/";
const CONFIG_SOURCE_PATH: &str = "~/Work/Configs/vscode/";

const CONFIG_FILES: [&str; 3] = ["setting.json", "keybinding.json", ""];

pub fn run() {
    h1("VS Code install and setup");

    h2("Installing VS Code");
    os::install("code");

    h2("Installing VS Code Extensions");
    for ext in EXTENSIONS {
        if ext.is_empty() {
            continue;
        }
        exe(ext, false);
    }
    h2("Creating configs symbolic links");
    for file in CONFIG_FILES {
        if file.is_empty() {
            continue;
        }
        files::slink(
            &format!("{CONFIG_SOURCE_PATH}/{file}"),
            &format!("{CONFIG_PATH}/{file}"),
        );
    }
}
