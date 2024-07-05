mod install;
mod create_mod;
mod create_app;

use clap::{Command, Arg, ArgMatches};

const SOURCE_DIR: &str = "src";

pub const RUST: &str = "rust";

#[must_use]
pub fn commands() -> Command {
    Command::new(RUST)
        .about("Rust programming language commands")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("install")
                .about("Rust programming languages and ecosystem")
                .arg(crate::arg_no_confirm()),
        )
        .subcommand(mod_commands())
        .subcommand(app_commands())
        .arg(crate::arg_version())
    }


pub fn handle(arg_matches: &ArgMatches) {
    match arg_matches.subcommand() {
        Some(("install", sub_arg_matches)) => 
            install::run(sub_arg_matches.get_flag("noconfirm")),
        Some(("mod", sub_arg_matches)) => mod_handle(sub_arg_matches),
        Some(("app", sub_arg_matches)) => app_handle(sub_arg_matches),
        
        _ => {
            if arg_matches.get_flag("version") {
                println!("Rust version: {}", env!("CARGO_PKG_VERSION"));
            } else {
                eprintln!("No valid subcommand found for rust");
            }
        }
    }
}

#[must_use]  // modules commands
pub fn mod_commands() -> Command {
    Command::new("mod")
        .about("Creating module in Rust Project")
        .arg_required_else_help(true)
        .arg(
            Arg::new("module_name")
                .help("The New Module name")
                .required(true)
                .index(1),
        )
}


pub fn mod_handle(arg_matches: &ArgMatches) {
    if let Some(module_name) = arg_matches.get_one::<String>("module_name") {
    println!("Executing the 'mod' command.");
        create_mod::run(SOURCE_DIR, module_name);
    } else {
        eprintln!("Error: Command argument is required");
    }
}


#[must_use]  //Applications commands
pub fn app_commands() -> Command {
    Command::new("app")
        .about("Creating New Application in Rust Project")
        .arg_required_else_help(true)
        .arg(
            Arg::new("app_name")
                .help("The New Application name")
                .required(true)
                .index(1),
        )
}


pub fn app_handle(arg_matches: &ArgMatches) {
    if let Some(app_name) = arg_matches.get_one::<String>("app_name") {
    println!("Executing the 'mod' command.");
        create_app::run(app_name);
    } else {
        eprintln!("Error: Command argument is required");
    }
}