extern crate exe as app;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Bash Command Beautiful Executor")
        .version("0.1.0")
        .about("Executes a bash command with an beautiful interface")
        .arg_required_else_help(true)
        .arg(
            Arg::new("command")
                .help("The Bash command to execute")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("noconfirm")
                .help("Skips confirmation")
                .short('n')
                .long("noconfirm")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();
    
    let command = matches.get_one::<String>("command").unwrap();
    let noconfirm_flag = matches.get_flag("noconfirm");
    app::exec(command, noconfirm_flag);
}