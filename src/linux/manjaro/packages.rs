use std::process::Command as ShellCommand;
use std::str;

use crate::prelude::*;

pub fn update() {
    H1!("System update with pamac");
    if check() {
        return;
    };
    exe!("sudo pamac upgrade -a --no-confirm");
    update();
}

pub fn install(packages: &str) {
    update();
    h2!("Installing packages for mangaro");
    exe!("sudo pamac install {packages} --no-confirm ");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        update();
    }

    #[test]
    fn test_install() {
        install("gimp vlc");
    }

    #[test]
    fn test_check_output_up_to_date() {
        assert!(check());
    }
}
