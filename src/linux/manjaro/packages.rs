use std::process::Command as ShellCommand;
use std::str;

use crate::prelude::*;

pub fn update() {
    H1!("System update");
    if check() {
        return;
    };
    exe!("sudo pamac upgrade -a --no-confirm", false);
    update();
}

pub fn install(packages: &str) {
    update();
    h2!("Installing");
    let cmd = format!("sudo pamac install {packages} --no-confirm ");
    exe!(&cmd, false);
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
