use crate::styles::{h1, h2};
use std::{
    fs::{self, File},
    path::Path,
    process
};

const SOURCE_DIR: &str = "src";


pub fn run (module_name: &str) {
    h1("Creating new module in Rust project");
    create_module_directory(SOURCE_DIR, module_name);
    create_module_directory(&format!("../{SOURCE_DIR}"), module_name);
    
}

fn directory_exists(path_string: &str ) -> bool {
    let path = Path::new(path_string);
    path.is_dir()
}

fn create_module_directory(target_dir: &str, module_name: &str){
    h2("Trying to create {module_name} in {target_dir}");
    if !directory_exists(target_dir) {
        println!("Target Directory '{target_dir}' does not exist.", );
        return;   
    }
    let module_path = Path::new(target_dir).join(module_name);

    match fs::create_dir(&module_path) {
        Ok(()) => {
            println!("Successfully created module directory: {module_path:?}");
            create_module_file(&module_path);
            process::exit(0);
        }
        Err(e) => eprintln!("Failed to create module directory: {e}"),
    }
}

fn create_module_file(target_path: &Path){
    let file_path = target_path.join("mod.rs");

    match File::create(&file_path) {
        Ok(_) => println!("Successfully created file: {file_path:?}"),
        Err(e) => eprintln!("Failed to create file: {e:?}"),
    }
}