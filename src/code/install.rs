use crate::{home_dir, prelude::*};
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

const VSCODE_CONFIG_PATH: &str = ".config/Code - OSS/User/";
const VSCODE_CONFIG_SOURCE_PATH: &str = "Work/Configs/vscode/";

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

    for file in CONFIG_FILES {
        if file.is_empty() {
            continue;
        }
        files::slink(
            &Path::new(&home_dir!()).join(VSCODE_CONFIG_SOURCE_PATH).join(file),
            &Path::new(&home_dir!()).join(VSCODE_CONFIG_PATH).join(file)
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