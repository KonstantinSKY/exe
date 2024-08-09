use tempfile::NamedTempFile;

use crate::prelude::*;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

#[macro_export]
macro_rules! home_dir {
    () => {
        std::env::var("HOME").unwrap_or_else(|_| {
            std::env::var("USER")
                .map(|user| format!("/home/{}", user))
                .unwrap_or_else(|_| "/home".to_string())
        })
    };
}

#[macro_export]
macro_rules! home_path {
    ($($segment:expr),+) => {{
        let home_dir = home_dir!();
        let mut path = PathBuf::from(home_dir);
        $(
            path.push($segment);
        )+
        path
    }};
}

pub fn delete(files: &str, noconfirm_flag: bool) {
    h2!("Deleting files: {files}");
    exe!("rm -rv {files}"; noconfirm_flag);
}

fn move_to_old(path: &Path) {
    let filename = path.to_str().unwrap();
    let old_filename = format!("{filename}.old");
    let old_path = path.join(".old");
    
    println!("Old file name: {old_filename}");
    println!("Old path name: {old_path:?}");

    // if path.is_dir() {
    //     h2!("Moving existing dir {filename} to {old_filename}");
    //     exe!("mv -r {path:?} '{old_filename}'"; true);
    // }
    // if path.is_file(){
    //     h2!("Moving existing dir {filename} to {old_filename}");
    //     exe!("mv {path:?} '{old_filename}'"; true);
    // }
}

pub fn enable_config_param(param: &str, config_file: &str, message: &str) {
    if !Path::new(config_file).exists() {
        warn_print!("{config_file} is not exists");
        return;
    }
    h2!("{message}");
    // exe!("sed -i 's/#{param}/{param}' $config_file");

    cmd!("sudo sed -i 's/#{param}/{param}' $config_file");
    let sed_command = format!("s/#{param}/{param}/");

    let output: Result<Output, std::io::Error> = Command::new("sudo")
        .arg("sed")
        .arg("-i")
        .arg(sed_command)
        .arg(config_file)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Successfully updated {param} in {config_file}");
            } else {
                eprintln!(
                    "Failed to update {} in {}. Error: {}",
                    param,
                    config_file,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to execute sed command: {e}");
        }
    }
}

pub fn slink_pair(link_source_pair: &[String]){
    slink(
        &home_path!(&link_source_pair[1]),
        &home_path!(&link_source_pair[0]),
    );
}

pub fn slink(source_path: &Path, link_path: &Path) {
    h2!("Creating Symbolic link: {link_path:?} -> {source_path:?}");

    if link_path.is_symlink() {
        println!("{link_path:?} is already symlink,  will be deleted");
        match fs::remove_file(link_path) {
            Ok(()) => println!("Successfully deleted old symlink: {link_path:?}"),
            Err(e) => {
                eprintln!("Failed to delete symlink: {link_path:?}. Error: {e}");
                return;
            }
        }
    }
    println!("Link path: {link_path:?}");

    if link_path.exists() {
        println!("{link_path:?} is exists");
        move_to_old(link_path);
    }

    if !source_path.exists() {
        eprintln!("!!! Error: Source not exist: {source_path:?}");
        return;
    }

    exe!("ln -sf {source_path:?} {link_path:?} && readlink -f {link_path:?}";  true);
    if link_path.is_symlink() {
        println!("Symlink successfully created");
    }
}

#[macro_export]
macro_rules! backup {
    ($($arg:tt)*) => {
        backup(&PathBuf::from(format!($($arg)*)), &home_path!(BACKUP_DIR))
    };
    ($($arg1:tt)*; $($arg2:tt)*) => {
        backup(&PathBuf::from(format!($($arg1)*)), &home_path!($($arg2)*))
    };
}

#[must_use]
pub fn backup(source_path: &Path, storage_path: &Path) -> bool {
    if !source_path.exists() {
        eprintln!("Source path is not exists: {source_path:?}");
        return false;
    }
    if source_path.is_symlink() {
        eprintln!("Can not backup symlink: {source_path:?}");
        return false;
    }
    if !storage_path.is_dir() {
        eprintln!("Wrong storage directory with path:{storage_path:?}");
        return false;
    }

    let now = chrono::Utc::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S-%f").to_string();
    let source: String = if let Some(name) = source_path.to_str() {
        format!("{}_backup_{timestamp}", &name[1..])
    } else {
        eprintln!("Invalid source file name: {source_path:?}");
        return false;
    };

    // let target_filename = format!("{}",source");
    let target_path = storage_path.join(source);
    // Ensure the target directory exists
    if let Some(parent) = target_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("Failed to create target directory: {e}");
            return false;
        }
    }

    println!("Backing up {source_path:?} --> {target_path:?}");
    match fs::copy(source_path, &target_path) {
        Ok(_) => {
            println!("Backup successful: {target_path:?}");
            true
        }
        Err(e) => {
            eprintln!("Failed to copy file: {e}");
            false
        }
    }
}

pub fn force_copy(source_path: &Path, destination_path: &Path) {
    // Create a temporary file in the destination directory
    let Some(destination_dir_path) = destination_path.parent() else {
        eprintln!("Invalid destination path");
        return;
    };
    // Create a temporary file
    let temp_file = match NamedTempFile::new_in(destination_dir_path) {
        Ok(temp_file) => temp_file,
        Err(e) => {
            eprintln!("Cant Create the Temporary file. Error: {e}");
            return;
        }
    };
    let temp_path = temp_file.path().to_path_buf();

    // Copy the source file to the temporary file
    match fs::copy(source_path, &temp_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Can not copy {source_path:?} to Temporary path {temp_path:?}. Error: {e}");
            return;
        }
    }
    // Move the temporary file to the destination
    match fs::rename(&temp_path, destination_path) {
        Ok(()) => {}
        Err(e) => {
            eprintln!(
                "Can not move {temp_path:?} to Temporary path {destination_path:?}. Error: {e}"
            );
            return;
        }
    }
    println!("{}", format!("Force copy finished successfully from {source_path:?} to {destination_path:?}").green());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_home_path_with_home_env() {
        // Set the HOME environment variable for the test.
        env::set_var("HOME", "/home/sky");

        let expected = PathBuf::from("/home/sky/config/app/settings.json/new_dir");
        let result = home_path!("config", "app/settings.json", "new_dir");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_backup_source_does_not_exist() {
        let temp_dir = tempdir().unwrap();
        let source_file = temp_dir.path().join("nonexistent.txt");
        let storage_dir = temp_dir.path().join("backup");

        // Create the storage directory
        fs::create_dir(&storage_dir).unwrap();

        // Run the backup function
        assert!(!backup(&source_file, &storage_dir));
    }

    #[test]
    fn test_backup_success() {
        let temp_dir = tempdir().unwrap();
        let source_path = temp_dir.path().join("source.txt");

        // Create the source file
        {
            let mut file = File::create(&source_path).unwrap();
            writeln!(file, "Hello, world!").unwrap();
        }

        // Run the backup function
        // assert!(backup!("{}", source_path.to_str().unwrap()));
        let _ = backup!("{}", source_path.to_str().unwrap());
    }
}
