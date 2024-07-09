pub mod os;
pub mod manjaro;
pub mod work_drive;




use clap::{ArgMatches, Command};

#[must_use]
pub fn commands() -> Command {
    Command::new("linux")
        .about("Linux commands")
        .arg_required_else_help(true)
        .subcommand(Command::new("mount_work").about("Mount Work Disk to Work Directory"))
}

pub fn handle(arg_matches: &ArgMatches) {
    match arg_matches.subcommand() {
        Some(("mount_work", _sub_matches)) => work_drive::mount(),
        _ => eprintln!("No valid subcommand found"),
    }
}
