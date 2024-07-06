use std::process::Command as ShellCommand;
use std::str;

use crate::{
    sh::exec::{exe, print_cmd},
    styles::{h1, h2},
};
pub fn run() {
    h1("System update");
    if  check() {
        return;
    };
    exe("sudo pamac upgrade -a --no-confirm", false)
}

const UP_TO_DATE_MESSAGE: &str = "Your system is up to date.";

fn check() -> bool {
    h2("Checking for update");
    print_cmd("pamac checkupdates -a");

    let output = ShellCommand::new("pamac")
        .arg("checkupdates")
        .arg("-a")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        // Convert the stdout bytes to a string
        let stdout = str::from_utf8(&output.stdout).unwrap();

        // Check for the specific string
        if stdout.contains(UP_TO_DATE_MESSAGE) {
            println!("{UP_TO_DATE_MESSAGE}");
            return true;
        }
        println!("{stdout}");
    } else {
        // Convert the stderr bytes to a string and print it
        let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8 sequence");
        eprintln!("Error:\n{}", stderr);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
    }

    #[test]
    fn test_check_output_up_to_date() {
        assert!(check());
    }
}
