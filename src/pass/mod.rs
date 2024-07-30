pub mod install;
pub mod config;


use clap::{ArgMatches, Command};

#[must_use]
pub fn commands() -> Command {
    Command::new("pass")
        .about("Pass Password manager commands")
        .arg_required_else_help(true)
        .subcommand(Command::new("install").about("Pass install and setup"))

}

pub fn handle(arg_matches: &ArgMatches) {
    match arg_matches.subcommand() {
        Some(("install", _sub_matches)) => install::run(),
        _ => eprintln!("No valid subcommand found"),
    }
}