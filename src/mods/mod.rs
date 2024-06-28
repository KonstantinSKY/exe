mod create;

use clap::{Command, Arg, ArgMatches};


#[must_use]
pub fn commands() -> Command {
    Command::new("mod")
        .about("Creating module in Rust Project")
        .arg_required_else_help(true)
        .arg(
            Arg::new("module_name")
                .help("The New Module name")
                .required(true)
                .index(1),
        )
}


pub fn handle(arg_matches: &ArgMatches) {
    if let Some(module_name) = arg_matches.get_one::<String>("module_name") {
    println!("Executing the 'mod' command.");
        create::run(module_name);
    } else {
        eprintln!("Error: Command argument is required");
    }
}
