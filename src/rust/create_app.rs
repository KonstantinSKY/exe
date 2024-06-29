use crate::styles::{h1, h2};
use std::path::Path;

use super::create_mod;

pub fn run (app_name: &str) -> bool {
    h1("New Application in Rust project");
    
    if !create_mod::run(super::SOURCE_DIR, app_name){
        return false;
    }
    let app_path = Path::new(super::SOURCE_DIR).join(app_name);
    let app_dir:&str =app_path.to_str().unwrap();
    h2("Creating module in app: models");
    create_mod::run(app_dir,  "models");
    h2("Creating module in app: routes");
    create_mod::run(app_dir,  "routes");
    h2("Creating module in app: repositories");
    create_mod::run(app_dir,  "repositories");
    h2("Creating module in app: services");
    create_mod::run(app_dir,  "services");
    true
} 