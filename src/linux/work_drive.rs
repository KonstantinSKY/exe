use crate::prelude::*;
use std::{env, str, fs};
use std::process::Command;

const LABEL: &str = "Work";
const DIR: &str = "Work";
const FS_TYPE: &str = "btrfs";
const OPTIONS: &str = "defaults";
const DUMP: &str = "0";
const PASS: &str = "2";
const FSTAB_PATH: &str = "/etc/fstab";

pub fn mount() {
    let dir = env::var("HOME").unwrap_or_else(|_| String::new()) + "/" + DIR;
    H1!("Work Drive with Label: {LABEL} and Directory: {dir}");

    h2!("Finding the device associated with label {LABEL}");
    cmd!("sudo findfs LABEL={LABEL}");

    let drive = if let Some(drive) = find_device(LABEL) {
        println!("Using drive: {drive}");
        drive
    } else {
        println!("No valid drive found.");
        return;
    };

    h2!("Creating work directory: {dir}");
    exe!(&format!("mkdir -p {dir}; ls -la {dir}"));

    h2!("Mounting {drive} to directory: {DIR}");
    exe!(&format!("sudo mount '{drive}' '{DIR}'"));

    h2!("Checking mounted drive: {drive}");
    exe!(&format!("mount | grep {DIR}"));
    
    let user = env::var("USER").unwrap_or_else(|_| String::new());
    h2!("Setting full access to mounted drive for user: {user}");
    exe!(&format!("sudo chown -R {user}:{user} {dir} && sudo chmod -R 700 {dir} && ls -la {dir}"));


    H1!("Change fstab file for auto mounting");
 
    h2!("Retrieving the UUID of the drive: {drive}");
    let uuid = if let Some(uuid) = get_uuid(&drive) {
        println!("UUID: {uuid}");
        uuid
    } else {
        eprintln!("No valid uuid found.");
        return;
    };

    let fstab_content = fs::read_to_string(FSTAB_PATH).expect("Unable to read /etc/fstab");
    if fstab_content.contains(&uuid) {
        println!("An entry for this UUID ({uuid}) already exists in /etc/fstab.");
        return;
    }
    // Backup the current fstab
    h2!("Backup the current fstab");
    exe!(&format!("cp {FSTAB_PATH} {FSTAB_PATH}.bak"));

    // Add the entry to fstab
    println!("h2 Adding the entry to fstab");
    let new_entry = format!("UUID={uuid} {dir} {FS_TYPE} {OPTIONS} {DUMP} {PASS}\n");

    h2!("Adding the entry to fstab: {new_entry}");
    exe!(&format!("echo {new_entry} | sudo tee -a /etc/fstab > /dev/null"));

    h2!("Reviewing the file {FSTAB_PATH}");
    exe!(&format!("cat {FSTAB_PATH}"));

    h2!("System rebooting");
    exe!("sudo reboot");

}


fn find_device(label: &str) -> Option<String> {
    let output = Command::new("sudo")
        .arg("findfs")
        .arg(format!("LABEL={label}"))
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let drive = str::from_utf8(&output.stdout).unwrap_or("").trim();
            if drive.is_empty() {
                eprintln!("No drive found with label: {label}");
                None
            } else {
                println!("Drive found: {drive}");
                Some(drive.to_string())
            }
        }
        Ok(output) => {
            eprintln!(
                "findfs command failed with error: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            None
        }
        Err(e) => {
            eprintln!("Failed to execute findfs: {e}");
            None
        }
    }
}

fn get_uuid(drive: &str) ->Option<String>{
    let output = Command::new("sudo")
    .arg("blkid")
    .arg("-s")
    .arg("UUID")
    .arg("-o")
    .arg("value")
    .arg(drive)
    .output();

    match output {
        Ok(output) if output.status.success() => {
            match str::from_utf8(&output.stdout) {
                Ok(uuid) => {
                    let uuid = uuid.trim().to_string();
                    if uuid.is_empty() {
                        eprintln!("UUID not found for drive: {drive}");
                        None
                    } else {
                        Some(uuid)
                    }
                }
                Err(err) => {
                    eprintln!("Failed to parse UUID output for drive {drive}: {err}");
                    None
                }
            }
        }
        Ok(output) => {
            eprintln!("blkid command failed for drive {drive}: {}", String::from_utf8_lossy(&output.stderr));
            None
        }
        Err(err) => {
            eprintln!("Failed to execute blkid command for drive {drive}: {err}");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_device() {
        find_device(LABEL);
    }
}
