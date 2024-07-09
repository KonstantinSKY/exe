pub mod os;
pub mod manjaro;
pub mod work_drive;




use clap::{ArgMatches, Command};

#[must_use]
pub fn commands() -> Command {
    Command::new("linux")
        .about("linux commands")
        .arg_required_else_help(true)
        .subcommand(Command::new("mount_work_drive").about("Mount Work Drive"))
}

pub fn handle(arg_matches: &ArgMatches) {
    match arg_matches.subcommand() {
        Some(("mount_work_drive", _sub_matches)) => work_drive::mount(),
        _ => eprintln!("No valid subcommand found"),
    }
}
