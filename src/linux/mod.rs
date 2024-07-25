pub mod manjaro;
pub mod os;
pub mod sync;
pub mod work_drive;

use clap::{Arg, ArgAction, ArgMatches, Command};
use manjaro::packages;

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

#[must_use] //Applications commands
pub fn add_commands() -> Command {
    Command::new("add").about("Install new package(s) to Linux system")
    .arg_required_else_help(true)
    .arg(
        Arg::new("packages")
            .help("The package name(s). You can add many package in one time")
            .required(true)
            .action(ArgAction::Append),
    )
}

pub fn add_handle(arg_matches: &ArgMatches) {
    if let Some(packages) = arg_matches.get_many::<String>("packages") {
        for package in packages {
            // println!("Installing package: {package}");
            os::install(package);
        }
        } else {
            eprintln!("Error: Command argument is required");
        }
}