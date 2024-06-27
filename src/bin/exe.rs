extern crate exe as app;

use clap::Command;

fn main() {
    // styles::app_started();

    let matches = Command::new("Universal Beautiful Executor")
        .version("0.1.0")
        .about("Executes a bash command with an beautiful interface")
        .arg_required_else_help(true)
        .subcommand(app::sh::commands())
        .subcommand(app::rust::commands())

        .get_matches();

    match matches.subcommand() {
        Some(("sh", arg_matches)) => handle_sh_subcommand(arg_matches),
        Some(("rust", arg_matches)) => handle_rust_subcommand(arg_matches),
        _ => println!("No Function Found"),
    }
    
}

// pub fn arg_no_confirm() -> Arg {
//     Arg::new("noconfirm")
//     .help("Skip confirmation flag")
//     .short('n')
//     .long("noconfirm")
//     .action(clap::ArgAction::SetTrue)
// }
