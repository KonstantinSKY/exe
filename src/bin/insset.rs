extern crate exe as app;

use clap::{Arg, Command};

fn main() {

    let matches = Command::new("insset")
        .version("0.1.0")
        .about("Installation and setting util for Linux Apps and their ecosystems")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("rust")
                .about("Rust programming languages and ecosystem")
                .arg(
                    Arg::new("noconfirm")
                        .help("Skips confirmation")
                        .short('n')
                        .long("noconfirm")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("rust", arg_matches)) => run(arg_matches.get_flag("noconfirm")),
        _ => println!("No Function Found"),
    }
}
