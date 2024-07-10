
use crate::prelude::*;
use std::fs;
use std::path::Path;


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



fn move_to_old(path: &Path) {
    
    let filename = path.to_str().unwrap();
    let old_filename = format!("{filename}.old");

    // Check if the file exists
    if fs::metadata(filename).is_ok() {
        h2!("Moving existing file {filename} to {old_filename}");
        let cmd: &str = &format!("mv '{filename}' '{old_filename}'");
        exe!(cmd, true);
        // Execute the move command
    }
}

pub fn slink(source_path: &Path, link_path: &Path) {
    h2!("Creating Symbolic link: {link_path:?} -> {source_path:?}");
    
    if link_path.is_symlink(){
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

    if link_path.exists(){
        println!("{link_path:?} is exists");
        move_to_old(link_path);
    }
   
    if !source_path.exists(){
        eprintln!("!!! Error: Source not exist: {source_path:?}");
        return;
    }

    exe!(&format!("ln -sf {source_path:?} {link_path:?} && readlink -f {link_path:?}"),  true);
    if link_path.is_symlink(){
        println!("Symlink successfully created");
    }
}


