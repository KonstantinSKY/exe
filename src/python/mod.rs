pub mod install;

use clap::{ArgMatches, Command};

#[must_use]
pub fn commands() -> Command {
    Command::new("py")
        .about("Python ecosystem commands")
        .arg_required_else_help(true)
        .subcommand(Command::new("install").about("Python ecosystem install and setup"))
        // .subcommand(Command::new("update").about("Python ecosystem install and setup"))

}

pub fn handle(arg_matches: &ArgMatches) {
    match arg_matches.subcommand() {
        Some(("install", _sub_matches)) => install::run(),
        // Some(("update", _sub_matches)) => update::run(),
        _ => eprintln!("No valid subcommand found"),
    }
}
