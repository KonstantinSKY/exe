pub mod install;
pub mod sync;

use clap::{ArgMatches, Command};

#[must_use]
pub fn commands() -> Command {
    Command::new("code")
        .about("VS CODE editor commands")
        .arg_required_else_help(true)
        .subcommand(Command::new("install").about("Code and ecosystem installing"))
        .subcommand(Command::new("sync").about("Sync setting json files with git repository"))

}

pub fn handle(arg_matches: &ArgMatches) {
    match arg_matches.subcommand() {
        Some(("install", _sub_matches)) => install::run(),
        Some(("sync", _sub_matches)) => sync::run(),
        _ => eprintln!("No valid subcommand found"),
    }
}
