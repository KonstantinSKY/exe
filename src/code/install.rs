use crate::prelude::*;
use std::env;

use crate::linux::os;
use crate::sh::files;

pub fn run() {
    H1!("VS Code install and setup");

    h2!("Installing VS Code");
    os::install("code");

    h2!("Installing VS Code Extensions");
    for ext in VSCODE_EXTENSIONS {
        if ext.is_empty() {
            continue;
        }
        exe!("code --install-extension {ext}");
    }

    h2!("Creating configs symbolic links");
    for file in CONFIG_FILES {
        if file.is_empty() {
            continue;
        }
        files::slink(
            &home_path!(VSCODE_CONFIG_SOURCE_PATH, file),
            &home_path!(VSCODE_CONFIG_PATH, file),
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
