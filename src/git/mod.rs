pub mod install;

use clap::{ArgMatches, Command};

#[must_use]
pub fn commands() -> Command {
    Command::new("git")
        .about("GIT commands")
        .arg_required_else_help(true)
        .subcommand(Command::new("install").about("GIT install and setup"))

}

pub fn handle(arg_matches: &ArgMatches) {
    match arg_matches.subcommand() {
        Some(("install", _sub_matches)) => install::run(),
        _ => eprintln!("No valid subcommand found"),
    }
}