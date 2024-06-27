use clap::{Arg, Command};

#[must_use]
pub fn commands() -> Command {
    Command::new("sh")
        .about("Run any shell command inside double quoter")
        .arg_required_else_help(true)
        .arg(
            Arg::new("command")
                .help("The Bash command to execute")
                .required(true)
                .index(1),
        )
        .arg(crate::arg_no_confirm())
}

fn handle(arg_matches: &clap::ArgMatches) {
    let command = arg_matches.get_one::<String>("command").unwrap();
    let noconfirm_flag = arg_matches.get_flag("noconfirm");
    app::exec(command, noconfirm_flag);
}