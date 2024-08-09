use crate::prelude::*;
use crate::linux::os;
use crate::sh::files::slink;

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
    for file in &config.config_files {
        let link_file = home_path!(&config.config_dir[0], file);
        let target_file = home_path!(&config.config_dir[1], file);
        slink(&target_file, &link_file);
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
