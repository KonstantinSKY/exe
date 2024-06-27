mod install;
use clap::Command;

#[must_use]
pub fn commands() -> Command {
    Command::new("rust")
        .about("Rust programming language commands")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("install")
                .about("Rust programming languages and ecosystem")
                .arg(crate::arg_no_confirm()),
        )
}


pub fn handle(arg_matches: &clap::ArgMatches) {
    if let Some(("install", sub_arg_matches)) = arg_matches.subcommand() {
        let noconfirm_flag = sub_arg_matches.get_flag("noconfirm");
        install::run(noconfirm_flag);
    } else {
        println!("No subcommand found for rust");
    }
}