use files::backup;

use crate::prelude::*;
use crate::sh::files::enable_config_param;
use std::process::Command as ShellCommand;
use std::str;


pub fn update() {
    H1!("System update with pamac");
    if check() {
        return;
    };
    exe!("sudo pamac upgrade -a --no-confirm");
    update();
}

pub fn install(packages: &str) {
    // update();
    h2!("Installing packages for Manjaro: {packages}");
    exe!("pamac info {packages} | grep -E 'Name|Version|Description' | awk '{{$1=$1;print}}'"; true);
    exe!("sudo pamac install {packages} --no-confirm ");
}

pub fn install_many(packages: &[&str]){
    for package in packages{
        if package.is_empty(){
            continue;
        }
        install(package);
    }
}

pub fn remove(packages: &str) {
    h2!("Removing packages for mangaro");
    exe!("sudo pamac remove {packages} --no-confirm ");
}

pub fn get_mirrors() {
    H1!("Repository mirrors update");

    h2!("Showing mirrors status");
    exe!("pacman-mirrors --status"; true);

    h2!("Searching and updating fastest");
    exe!("sudo pacman-mirrors --fasttrack");

    h2!("Showing New status of mirrors pool");
    exe! ("pacman-mirrors --status"; true);

    h2!("Fast update with pacman");
    exe!("sudo pacman -Suy --noconfirm");
}

// const UP_TO_DATE_MESSAGE: &str = "Your system is up to date.";

fn check() -> bool {
    h2!("Checking for update");
    cmd!("pamac checkupdates -a");

    let output = ShellCommand::new("pamac")
        .arg("checkupdates")
        .arg("-a")
        .output()
        .expect("Failed to execute command");
    output.status.success()
}

pub fn enable_aur() {
    H1!("PAMAC & AUR (ADVANCED USER REPOSITORY) SETUP in $CONFIG");
    if !backup!("{PAMAC_CONFIG}") {return;}
    h2!("Updating config file: {PAMAC_CONFIG}");
    
    let ecp = enable_config_param;
    ecp(
        "EnableAUR",
        PAMAC_CONFIG,
        "Allow Pamac to search and install packages from AUR",
    );
    ecp(
        "CheckAURUpdate",
        PAMAC_CONFIG,
        "When AUR support is enabled check for updates from AUR",
    );
    ecp("RemoveUnrequiredDeps",
        PAMAC_CONFIG, 
        "When removing a package, also remove those dependencies that are not required by other packages (recurse option)");
    ecp(
        "NoUpdateHideIcon",
        PAMAC_CONFIG,
        "When no update is available, hide the tray icon",
    );
    ecp(
        "DownloadUpdates",
        PAMAC_CONFIG,
        "Download updates in background",
    );

    h2!("Showing config file {PAMAC_CONFIG}");
    exe!("sudo cat {PAMAC_CONFIG}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        update();
    }

    #[test]
    fn test_install() {
        install("unzip");
    }

    #[test]
    fn test_check_output_up_to_date() {
        assert!(check());
    }
}
