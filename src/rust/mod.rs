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
