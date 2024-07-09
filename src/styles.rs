#![allow(dead_code)]

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
