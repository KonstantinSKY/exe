use crate::sh::files::slink;
use crate::{home_dir, prelude::*, sh};
use std::fs;
use std::path::Path;
use std::process::Command;

use super::manjaro;

pub fn update() {
    match get().as_str() {
        "Manjaro" => manjaro::packages::update(),
        "Unknown" => println!("Unknown operating system"),
        _ => println!("OS not supported for update"),
    }
}

pub fn install(packages: &str) {
    match get().as_str() {
        "Manjaro" => manjaro::packages::install(packages),
        "Unknown" => println!("Unknown operating system"),
        _ => println!("OS not supported for install"),
    }
}

pub fn mirrors() {
    match get().as_str() {
        "Manjaro" => manjaro::packages::get_mirrors(),
        "Unknown" => println!("Unknown operating system and not supported for mirrors"),
        _ => println!("OS not supported for Mirrors"),
    }
}

pub fn setup() {
    H1!("Linux common setup");

    h2!("Cloning config repository to Work directory");
    // let home_dir = get_home_dir();
    // let home_dir_path = Path::new(&home_dir);
    let work_path = home_path!(WORK_DIR);
    let configs_path = home_path!(CONFIGS_DIR);

    exe!("git clone {CONFIG_REPO} {configs_path:?}; ls -la {configs_path:?}"; true);


    // Getting configs 

    h2!("Creating symbolic links to main directories");
    for dir in MAIN_DIRS {
        if dir.is_empty() {
            continue;
        }
        println!("For {}", dir.green());
        let link_path = home_path!(dir);
        let source_path = work_path.join(dir);

        if is_empty_dir(&link_path) {
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
        sh::files::slink(&source_path, &link_path);
    }
    run!(setup_rc, "Setting RC files for all shell");
    run!(fonts, "Font Setting");

    manjaro::setup::run();
    // match get().as_str() {
        // "Manjaro" => manjaro::setup::run(),
        // "Unknown" => println!("Unknown operating system"),
        // _ => println!("OS not supported for install"),
    // }
}

pub fn is_empty_dir(path: &Path) -> bool {
    if !path.is_dir() {
        return false;
    };

    match fs::read_dir(path) {
        Ok(mut entries) => entries.next().is_none(),
        Err(_) => false,
    }
}

fn setup_rc() {
    H1!("Setting up rc files");
    let rc_path = home_path!(MAIN_RC);

    let include_string = format!(". {}", rc_path.to_str().unwrap());
    println!("include string: {include_string}");
    for &rc_file in &RC_FILES {
        h2!("\n For {}", rc_file.green());
        let target_file = format!(".{rc_file}");
        let target_path = home_path!(target_file);
        
        if !target_path.exists() {
            exe!("touch {target_path:?}");
        }
        if !target_path.is_file() {
            println!("{target_path:?} is not file, skipping)");
            continue;
        }
        let file_content = match fs::read_to_string(&target_path) {
            Ok(content) => content,
            Err(e) => {
                println!("Unable to read file. Error: {e}");
                continue;
            }
        };
        h2!("Checking if link string  {include_string} is already in {target_path:?}");
        // println!("{file_content}");
        if file_content.contains(&include_string) {
            println!("The file {target_path:?} already has: {include_string}");
            continue;
        };

        h2!("Adding link string  {include_string} to {target_path:?}");
        exe!("echo {include_string} | tee -a {target_path:?}");

        h2!("Checking if added");
        exe!("tail -n 2 {target_path:?}"; true);
    }
}

pub fn fonts() {
    H1!("Additional fonts");

    let local_font_path = home_path!(LOCAL_FONT_DIR);
    let config_font_path = home_path!(WORK_DIR, CONFIG_FONT_DIR);

    slink(&config_font_path, &local_font_path);

    h2!("Clearing fontconfig  cache");
    exe!("rm -rf ~/.cache/fontconfig/*");

    h2!("Updating fonts cache");
    exe!("fc-cache -fv {local_font_path:?}");
}

fn get() -> String {
    // Run the hostnamectl command and capture the output
    let hostname = Command::new("hostnamectl")
        .output()
        .expect("Failed to execute command");

    let hostname_str = String::from_utf8_lossy(&hostname.stdout);
    for line in hostname_str.lines() {
        if line.contains("Operating System") {
            println!("{line}");
        }
    }

    let os_name = match hostname_str {
        s if s.contains("Ubuntu") => "Ubuntu",
        s if s.contains("Manjaro") => "Manjaro",
        _ => "Unknown", // Default case
    };

    // println!("OS: {os_name}");
    os_name.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_os() {
        let os = get();
        println!("Identified OS in test: {os}");

        // Asserting the OS is either "Ubuntu" or "Manjaro" or "Unknown" as per current implementation
        assert!(os == "Ubuntu" || os == "Manjaro" || os == "Unknown");
    }

    #[test]
    fn test_install() {
        install("gimp partitionmanager");
    }

    #[test]
    fn test_setup_rc() {
        setup_rc();
    }
}
