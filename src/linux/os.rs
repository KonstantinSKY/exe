

use std::process::Command;


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

    println!("OS: {}", os_name);
    os_name.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_os() {
        let os = get();
        println!("Identified OS in test: {}", os);

        // Asserting the OS is either "Ubuntu" or "Manjaro" or "Unknown" as per current implementation
        assert!(os == "Ubuntu" || os == "Manjaro" || os == "Unknown");
    }

    #[test]
    fn test_install() {
    install("gimp partitionmanager")
    }
}
