use crate::prelude::*;
use std::{
    fs::{self, File},
    path::Path,
    };


pub fn run (source_dir: &str, module_name: &str) -> bool {
    H1!("New module in Rust project");
    
    let source_path = Path::new(source_dir);
    if !source_path.is_dir(){
        println!("Target Directory '{source_path:?}' does not exist.");
        return false;
    }

    let module_path =source_path.join(module_name);

    h2!("Creating module directory: {:?}", &module_path);
    match fs::create_dir(&module_path) {
        Ok(()) => {
            println!("Successfully created module directory: {module_path:?}");
            create_module_file(&module_path);
            true
        }
        Err(e) => {
            eprintln!("Failed to create module directory: {e}");
            false
        }
    }
}


fn create_module_file(target_path: &Path){
    h2!("Creating file mod.rs");
    let file_path = target_path.join("mod.rs");

    match File::create(&file_path) {
        Ok(_) => println!("Successfully created file: {file_path:?}"),
        Err(e) => eprintln!("Failed to create file: {e:?}"),
    }
}