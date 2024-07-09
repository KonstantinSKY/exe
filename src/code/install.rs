use crate::prelude::*;
use std::env;

use crate::linux::os;
use crate::sh::files;

const EXTENSIONS: [&str; 10] = [
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

const CONFIG_PATH: &str = ".config/Code - OSS/User/";
const CONFIG_SOURCE_PATH: &str = "Work/Configs/vscode/";

const CONFIG_FILES: [&str; 3] = ["settings.json", "keybindings.json", ""];

pub fn run() {
    H1!("VS Code install and setup");

    h2!("Installing VS Code");
    os::install("code");

    h2!("Installing VS Code Extensions");
    for ext in EXTENSIONS {
        if ext.is_empty() {
            continue;
        }
        exe!(&format!("code --install-extension {ext}"), false);
    }
    h2!("Creating configs symbolic links");

    let home_dir = env::var("HOME").unwrap_or_default();
    for file in CONFIG_FILES {
        if file.is_empty() {
            continue;
        }
        files::slink(
            &format!("{home_dir}/{CONFIG_SOURCE_PATH}{file}"),
            &format!("{home_dir}/{CONFIG_PATH}{file}"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
    }
}