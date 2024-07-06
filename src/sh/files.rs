use super::exec::exe;
use crate::styles::h2;
use std::fs;
use std::path::Path;

pub fn move_to_old(filename: &str) {
    let old_filename = format!("{filename}.old");

    // Check if the file exists
    if fs::metadata(filename).is_ok() {
        h2(&format!(
            "Moving existing file {filename} to {old_filename}",
        ));
        let cmd: &str = &format!("mv {filename} {old_filename}");
        exe(cmd, true);
        // Execute the move command
    }
}

pub fn slink(source: &str, link: &str) {
    h2(&format!("Creating Symbolic link: {link} -> {source}"));
    let link_path = Path::new(link);
    if link_path.is_symlink(){
        println!("{link} is already symlink,  will be deleted");
        match fs::remove_file(link_path) {
            Ok(()) => println!("Successfully deleted symlink: {link}"),
            Err(e) => {
                eprintln!("Failed to delete symlink: {link}. Error: {e}");
                return;
            }
        }
    }
    if link_path.exists(){
        move_to_old(link);
    }
    
    if !Path::new(source).exists(){
        eprintln!("Error: Source not exist: {source}");
        return;
    }

    exe(&format!("ln -sf {source} {link} && readlink -f '$symlink_path')"),  false);
    if link_path.is_symlink(){
        println!("Symlink successfully created");
    }
}
