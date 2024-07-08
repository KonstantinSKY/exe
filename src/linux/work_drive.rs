use crate::prelude::*;
use crate::{
    sh::exec::{exe, print_cmd},
    // linux::os,
    // styles::{h1, h2},
    H1,
};
use std::process::Command;
use std::str;
use std::env;

const LABEL: &str = "Work";
const DIR: &str = "Work";
const FS_TYPE: &str = "btrfs";
const OPTIONS: &str = "defaults";
const DUMP: &str = "0";
const PASS: &str = "2";

pub fn mount() {
    let dir = env::var("HOME").unwrap_or_else(|_| String::new()) + "/" + DIR;
    H1!("Work Drive with Label: {LABEL} and Directory: {dir}");

    h2!("Finding the device associated with label {LABEL}");
    print_cmd(&format!("sudo findfs LABEL={LABEL}"));

    let output = Command::new("sudo")
        .arg("findfs")
        .arg(format!("LABEL={LABEL}"))
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let drive = str::from_utf8(&output.stdout).unwrap_or("").trim();
            if drive.is_empty() {
                eprintln!("No drive found with label: {LABEL}");
            } else {
                println!("Drive found: {drive}");
            }
        }
        Ok(output) => eprintln!(
            "findfs command failed with error: {}",
            String::from_utf8_lossy(&output.stderr)
        ),
        Err(e) => eprintln!("Failed to execute findfs: {e}"),
    }

    // println!("Drive with label: {LABEL} was found: {drive}");

    // h2(&format!("Creating work directory: {DIR}"));
    // exe(&format!("mkdir -p '{DIR}'; ls -la '{DIR}'"), false);

    // h2(&format!("Mounting {drive} to directory: {DIR}"));
    // exe(&format!("sudo mount '{drive}' '{DIR}'"), false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        mount();
    }
}
