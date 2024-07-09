
use crate::{prelude::*, sh};
use std::fs;
use std::process::Command;
use std::path::Path;


use super::manjaro;

pub fn update() {
    match get().as_str() {
        "Manjaro" => manjaro::packages::update(),
        "Unknown" => println!("Unknown operating system"),
        _ => println!("OS not supported for update"),
    }
}

pub fn install(packages: &str){
    match get().as_str() {
        "Manjaro" => manjaro::packages::install(packages),
        "Unknown" => println!("Unknown operating system"),
        _ => println!("OS not supported for install"),
    }
}

pub fn mirrors(){
    match get().as_str() {
        "Manjaro" => manjaro::packages::get_mirrors(),
        "Unknown" => println!("Unknown operating system"),
        _ => println!("OS not supported for install"),
    }
}

pub fn setup(){
    H1!("Linux setup");

    h2!("Cloning config repository to Work directory");
    let home_dir = env::var("HOME").unwrap_or_else(|_| String::new());
    let home_dir_path=Path::new(&home_dir);
    let work_dir_path = Path::new(&home_dir).join(WORK_DIR);
    let work_dir = work_dir_path.to_str().unwrap_or("");
    
    exe!(&format!("git clone {CONFIG_REPO} {work_dir}; ls -la {work_dir}"));
    h2!("Creating symbolic links to main directories");
    for dir in MAIN_DIRS {
        println!("For {}", dir.green());
        let link_path = home_dir_path.join(dir);
        let source_path = work_dir_path.join(dir);
        
        if is_empty_dir(&link_path) {
            println!("Found Empty Directory, will be deleted");
            exe!(&format!("rm {link_path:?} -r"), true);
        }
        if link_path.is_dir() {
            println!("Not empty directory: {link_path:?}, will be skip");
            continue;
        }
        if link_path.is_file() {
            println!("{dir} is file, will be skip");
            continue;
        }
        if source_path.exists() && !source_path.is_dir(){
            println!("Source path is Exists and not Directory, will be skipped");
            continue;
        }
        let source = source_path.to_str().unwrap_or("");
        let link = link_path.to_str().unwrap_or("");

        exe!(&format!("mkdir -p {source}"), true);        
        sh::files::slink(source, link);
    }
}
   
fn is_empty_dir(path: &Path) -> bool {
    if !path.is_dir(){
        return false;
    };

    match fs::read_dir(path) {
        Ok(mut entries) => entries.next().is_none(),
        Err(_) => false,
    }
}

fn get() -> String {
   // Run the hostnamectl command and capture the output
   let hostname = Command::new("hostnamectl")
   .output()
   .expect("Failed to execute command");

    let hostname_str = String::from_utf8_lossy(&hostname.stdout);
    println!("{hostname_str}"); 

    let os_name = match hostname_str {
        s if s.contains("Ubuntu") => "Ubuntu",
        s if s.contains("Manjaro") => "Manjaro",
        _ => "Unknown", // Default case
    };

    println!("OS: {os_name}");
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
}
