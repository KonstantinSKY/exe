use super::config::Config;
use crate::prelude::*;

use super::os::{mirrors, update};
use files::{slink, slink_pair};
use std::fs;

pub fn run() {
    H1!("Linux common setup");
    h2!("Cloning config repository to Work directory");
    // let home_dir = get_home_dir();
    // let home_dir_path = Path::new(&home_dir);
    let configs_path = home_path!(CONFIGS_DIR);

    exe!("git clone {CONFIG_REPO} {configs_path:?}; ls -la {configs_path:?}");

    run!(mirrors, "Setting mirrors for Linux");
    run!(update, "Full update Linux packages");

    run!(stop_beep_sound, "Stop PC beeper sound");

    run!(
        crate::alacritty::install,
        "Alacritty terminal install and setup"
    );
    
    let config = Config::new();
    run!(
        || create_symlinks(&config),
        "Creating SymLinks for Common Work Directories and Files"
    );
    // h2!("Creating symlinks to important file");
    // slink_pair(&config.profile);
    // slink_pair(&config.mimeapps_list);
    // slink_pair(&config.xresources);

    // run!(|| setup_rc(&config), "Setting RC files for all shell");
    // run!(|| fonts(&config), "Font Setting");
    // run!(|| trash(&config), "Setup trash-cli and trash-folder");
    // h2!(
    //     "Installing Linux common package collection: {:?}",
    //     config.packages
    // );
}

fn create_symlinks(cfg: &Config) {
    h2!("Creating File Symlinks");
    
    
    h2!("Creating Directory Symlinks");
    for dir in &cfg.work_dirs {
        if dir.is_empty() {
            continue;
        }
        println!("For {}", dir.clone().green());
        let link_path = home_path!(dir);
        let source_path = home_path!(WORK_DIR, dir);

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
        exe!("mkdir -vp {source_path:?}"; true);
        slink(&source_path, &link_path);
    }
}


fn fonts(cfg: &Config) {
    H1!("Fonts settings");
    slink_pair(&cfg.font_dir);
    update_fonts(cfg);
}

fn update_fonts(cfg: &Config) {
    h2!("Clearing fontconfig  cache");
    exe!("rm -rf ~/{}", cfg.font_cache_files);

    h2!("Updating fonts cache");
    exe!("fc-cache -fv {}", cfg.font_dir[0]);
}

fn trash(cfg: &Config) {
    super::os::install("trash-cli");
    let tp = home_path!(&cfg.trash_dir);
    h2!("Making Trash folder");
    exe!("mkdir -pv {tp:?}; trash --trash-dir {tp:?}"); // To do config

    h2!("Checking Trash Directory");
    exe!("ls -la {tp:?}; trash --directory"; true);
}

fn setup_rc(cfg: &Config) {
    H1!("Setting up rc files");

    let include_string = format!(". {}", cfg.rc);
    println!("Each rc files will include string: {include_string}");

    for rc_file in &cfg.rc_files {
        h2!("\n For {}", rc_file.clone().green());
        let rc_path = home_path!(rc_file);

        if !rc_path.exists() {
            exe!("touch {rc_path:?}");
        }
        if !rc_path.is_file() {
            println!("{rc_path:?} is not file, skipping)");
            continue;
        }
        let file_content = match fs::read_to_string(&rc_path) {
            Ok(content) => content,
            Err(e) => {
                println!("Unable to read file. Error: {e}");
                continue;
            }
        };
        h2!("Checking if link string  {include_string} is already in {rc_path:?}");
        // println!("{file_content}");
        if file_content.contains(&include_string) {
            println!("The file {rc_path:?} already has: {include_string}");
            continue;
        };

        h2!("Adding link string  {include_string} to {rc_path:?}");
        exe!("echo {include_string} | tee -a {rc_path:?}");

        h2!("Checking if added");
        exe!("tail -n 2 {rc_path:?}"; true);
    }
}

fn stop_beep_sound() {
    exe!("echo 'blacklist pcspkr' | sudo tee -a /etc/modprobe.d/nobeep.conf");
}
