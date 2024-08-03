
use crate::prelude::*;
use super::os::{mirrors, update};

use std::fs;
pub fn setup() {
    H1!("Linux common setup");
    
    
    h2!("Cloning config repository to Work directory");
    // let home_dir = get_home_dir();
    // let home_dir_path = Path::new(&home_dir);
    let configs_path = home_path!(CONFIGS_DIR);

    exe!("git clone {CONFIG_REPO} {configs_path:?}; ls -la {configs_path:?}");

    run!(mirrors, "Setting mirrors for Linux");
    run!(update, "Full update Linux packages");
    
    run!(crate::alacritty::install, "Alacritty terminal install and setup");
    
    run!(create_dir_symlinks, "Creating SymLinks for Common Work Directories");
    run!(setup_rc, "Setting RC files for all shell");
    run!(fonts, "Font Setting");
}

fn create_dir_symlinks(){
    let config = super::config::Config::new("linux");
    let work_path = home_path!(WORK_DIR);

    for dir in &config.work_dirs {
        if dir.is_empty() {
            continue;
        }
        println!("For {}", dir.clone().green());
        let link_path = home_path!(dir);
        let source_path = work_path.join(dir);

        if super::os::is_empty_dir(&link_path) {
            println!("Found Empty Directory, will be deleted");
            exe!("rm {link_path:?} -r"; true);
        }
        if link_path.is_dir() {
            println!("Not empty directory: {link_path:?}, will be skip");
            continue;
        }
        if link_path.is_file() {
            println!("{dir} is file, will be skip");
            continue;
        }
        if source_path.exists() && !source_path.is_dir() {
            println!("Source path is Exists and not Directory, will be skipped");
            continue;
        }
        // let source = source_path.to_str().unwrap_or("");
        // let link = link_path.to_str().unwrap_or("");

        exe!("mkdir -vp {source_path:?}"; true);
        crate::sh::files::slink(&source_path, &link_path);
    }
}


fn setup_rc() {
    let config = super::config::Config::new("linux");
    H1!("Setting up rc files");
    let rc_path = home_path!(config.rc);
    let include_string = format!(". {}", rc_path.to_str().unwrap());

    println!("include string: {include_string}");

    // for &rc_file in &config.rc_files {
    //     h2!("\n For {}", rc_file.green());
    //     let target_file = format!(".{rc_file}");
    //     let target_path = home_path!(target_file);
        
    //     if !target_path.exists() {
    //         exe!("touch {target_path:?}");
    //     }
    //     if !target_path.is_file() {
    //         println!("{target_path:?} is not file, skipping)");
    //         continue;
    //     }
    //     let file_content = match fs::read_to_string(&target_path) {
    //         Ok(content) => content,
    //         Err(e) => {
    //             println!("Unable to read file. Error: {e}");
    //             continue;
    //         }
    //     };
    //     h2!("Checking if link string  {include_string} is already in {target_path:?}");
    //     // println!("{file_content}");
    //     if file_content.contains(&include_string) {
    //         println!("The file {target_path:?} already has: {include_string}");
    //         continue;
    //     };

    //     h2!("Adding link string  {include_string} to {target_path:?}");
    //     exe!("echo {include_string} | tee -a {target_path:?}");

    //     h2!("Checking if added");
    //     exe!("tail -n 2 {target_path:?}"; true);
    // }
}

pub fn fonts() {
    H1!("Additional fonts");
    let config = super::config::Config::new("linux");
    let local_font_path = home_path!(LOCAL_FONT_DIR);
    let config_font_path = home_path!(WORK_DIR, CONFIG_FONT_DIR);

    crate::sh::files::slink(&config_font_path, &local_font_path);

    h2!("Clearing fontconfig  cache");
    exe!("rm -rf ~/.cache/fontconfig/*");

    h2!("Updating fonts cache");
    exe!("fc-cache -fv {local_font_path:?}");
}