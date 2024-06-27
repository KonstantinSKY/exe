pub mod exec;

use clap::{Arg, Command, ArgMatches};

#[must_use]
pub fn commands() -> Command {
    Command::new("sh")
        .about("Run any shell command inside double quoters")
        .arg_required_else_help(true)
        .arg(
            Arg::new("command")
                .help("The Bash command to execute")
                .required(true)
                .index(1),
        )
        .arg(crate::arg_no_confirm())
}

pub fn handle(arg_matches: &ArgMatches) {
    if let Some(command) = arg_matches.get_one::<String>("command") {
        let noconfirm_flag = arg_matches.get_flag("noconfirm");
        exec::exe(command, noconfirm_flag);
    } else {
        eprintln!("Error: Command argument is required");
    }
}