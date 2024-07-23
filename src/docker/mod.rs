pub mod install;

use clap::{ArgMatches, Command};

#[must_use]
pub fn commands() -> Command {
    Command::new("docker")
        .about("docker commands")
        .arg_required_else_help(true)
        .subcommand(Command::new("install").about("Docker and ecosystem installing"))
        .subcommand(Command::new("hello").about("Docker and ecosystem installing"))
}

pub fn handle(arg_matches: &ArgMatches) {
    match arg_matches.subcommand() {
        Some(("install", _sub_matches)) => install::run(),
        Some(("hello", _sub_matches)) => install::hello(),
        _ => eprintln!("No valid subcommand found"),
    }
}
