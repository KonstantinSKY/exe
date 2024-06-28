// use crate::styles::{h1, h2},
// };

use std::fs;
use std::path::Path;

const SOURCE_PATH: &str = "src";

pub fn directory_exists() -> bool {
    let path = Path::new(SOURCE_PATH);
    path.is_dir()
}

pub fn run (module_name: &str) {

    if !directory_exists() {
        println!("Directory '{SOURCE_PATH}' does not exist.", );
        return;
    }

    
    println!("Directory '{SOURCE_PATH}' exist.", );
        // Create the full path for the new module directory
        let module_path = Path::new(SOURCE_PATH).join(module_name);
    println!("New module path: {:?}", module_path);
        // Create the new module directory
        // if let Err(e) = fs::create_dir(&module_path) {
        //     eprintln!("Failed to create module directory: {}", e);
        // } else {
        //     println!("Successfully created module directory: {:?}", module_path);
        // }

}