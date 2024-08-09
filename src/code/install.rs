use crate::prelude::*;
use crate::linux::os;
use crate::sh::files::slink_pair;

pub fn run() {
    H1!("VS Code install and setup");
    let config = super::config::Config::new();
    h2!("Installing VS Code");
    os::install(&config.packages);

    h2!("Installing VS Code Extensions");
    for ext in &config.extensions {
        if ext.is_empty() {
            continue;
        }
        exe!("code --install-extension {ext}");
    }

    h2!("Creating configs symbolic links");
    slink_pair(&config.config_dir);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
    }
}
