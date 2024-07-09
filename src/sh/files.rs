
use crate::prelude::*;
use std::fs;
use std::path::Path;

pub fn move_to_old(filename: &str) {
    let old_filename = format!("{filename}.old");

    // Check if the file exists
    if fs::metadata(filename).is_ok() {
        h2!("Moving existing file {filename} to {old_filename}");
        let cmd: &str = &format!("mv '{filename}' '{old_filename}'");
        exe!(cmd, true);
        // Execute the move command
    }
}

pub fn slink(source: &str, link: &str) {
    h2!("Creating Symbolic link: {link} -> {source}");
    let link_path = Path::new(link);
    let source_path = Path::new(source);
    if link_path.is_symlink(){
        println!("{link} is already symlink,  will be deleted");
        match fs::remove_file(link_path) {
            Ok(()) => println!("Successfully deleted old symlink: {link}"),
            Err(e) => {
                eprintln!("Failed to delete symlink: {link}. Error: {e}");
                return;
            }
        }
    }
    println!("Link path: {link_path:?}");


    if link_path.exists(){
        println!("{link_path:?} is exists");
        move_to_old(link);
    }
   
    if !source_path.exists(){
        eprintln!("!!! Error: Source not exist: {source_path:?}");
        return;
    }

    exe!(&format!("ln -sf {source} {link} && readlink -f {link_path:?}"),  true);
    if link_path.is_symlink(){
        println!("Symlink successfully created");
    }
}


