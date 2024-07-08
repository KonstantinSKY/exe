#![allow(dead_code)]

use console::Style;
use crossterm::style::Stylize;
use std::env;

#[macro_export]
macro_rules! H1 {
    ($($arg:tt)*) => {
        println!("\n{}", format!($($arg)*).to_uppercase().white().bold());
    };
}

#[macro_export]
macro_rules! h2 {
    ($($arg:tt)*) => {
        println!("\n{} ...", format!($($arg)*).white().bold());
    };
}

#[macro_export]
macro_rules! cmd {
    ($($arg:tt)*) => {
        println!("{}: {}", "Command".blue(), format!($($arg)*).white());
    };
}
// pub trait ApplyStyle {
//     fn green(self) -> String;
//     fn cyan(self) -> String;
//     fn blue(self) -> String;
//     fn red(self) -> String;
//     fn yellow(self) -> String;
//     fn white(self) -> String;
//     fn white_bold(self) -> String;
//     fn magenta_bold(self) -> String;
//     fn bold(self) -> String;
// }

// impl ApplyStyle for &str {
//     fn green(self) -> String {
//         Style::new().green().apply_to(self).to_string()
//     }
//     fn cyan(self) -> String {
//         Style::new().cyan().apply_to(self).to_string()
//     }
//     fn blue(self) -> String {
//         Style::new().blue().apply_to(self).to_string()
//     }
//     fn red(self) -> String {
//         Style::new().red().apply_to(self).to_string()
//     }
//     fn yellow(self) -> String {
//         Style::new().yellow().apply_to(self).to_string()
//     }
//     fn white(self) -> String {
//         Style::new().white().apply_to(self).to_string()
//     }
//     fn bold(self) -> String {
//         Style::new().bold().apply_to(self).to_string()
//     }
//     fn white_bold(self) -> String {
//         Style::new().white().bold().apply_to(self).to_string()
//     }
//     fn magenta_bold(self) -> String {
//         Style::new().magenta().bold().apply_to(self).to_string()
//     }
// }

// pub fn h1(header_string: &str) {
//     println!("\n{}", header_string.to_uppercase().white_bold());
// }

// pub fn h2(header_string: &str) {
//     println!("\n{} ...", header_string.on_white() .white());
// }

// pub fn app_started() {

//     let text = "this ".on_white();
    
//     match env::args().next() {
//         Some(binary_name) => println!("Application {binary_name} started"),
//         None => println!("Failed to retrieve the application name."),
//     }
// }
