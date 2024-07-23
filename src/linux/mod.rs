pub mod manjaro;
pub mod os;
pub mod sync;
pub mod work_drive;

use clap::{ArgMatches, Command};

#[must_use]
pub fn commands() -> Command {
    Command::new("linux")
        .about("Linux commands")
        .arg_required_else_help(true)
        .subcommand(Command::new("mount_work").about("Mount Work Disk to Work Directory"))
        .subcommand(Command::new("update").about("Update Linux System (fast) "))
        .subcommand(Command::new("mirrors").about("Update Linux Repository mirrors"))
        .subcommand(Command::new("setup").about("Common Linux Setups"))
}

pub fn handle(arg_matches: &ArgMatches) {
    match arg_matches.subcommand() {
        Some(("mount_work", _sub_matches)) => work_drive::mount(),
        Some(("update", _sub_matches)) => os::update(),
        Some(("mirrors", _sub_matches)) => os::mirrors(),
        Some(("setup", _sub_matches)) => os::setup(),

        _ => eprintln!("No valid subcommand found"),
    }
}

#[must_use] //Applications commands
pub fn sync_commands() -> Command {
    Command::new("sync").about("Sync all Linux and Apps settings to GitHub")
}

pub fn sync_handle(arg_matches: &ArgMatches) {
    match arg_matches.subcommand() {
        Some(("sync", _sub_matches)) => sync::run(),
        _ => eprintln!("No valid subcommand found"),
    }
}
