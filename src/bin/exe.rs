extern crate exe as app;

use clap::Command;



fn main() {
    // styles::app_started();
    app::configs::init_config();



    let matches = Command::new("Universal Beautiful Executor")
        .version("0.1.0")
        .about("Executes a bash command with an beautiful interface")
        .arg_required_else_help(true)
        .subcommand(app::sh::commands())
        .subcommand(app::rust::commands())
        .subcommand(app::rust::mod_commands())
        .subcommand(app::rust::app_commands())
        .subcommand(app::deploy::commands())
        .subcommand(app::sh::kill_commands())
        .subcommand(app::code::commands())
        .subcommand(app::docker::commands())
        .subcommand(app::linux::commands())
        .subcommand(app::linux::sync_commands())
        .subcommand(app::megasync::commands())
        .get_matches();

    match matches.subcommand() {
        Some(("sh", arg_matches)) => app::sh::handle(arg_matches),
        Some((app::rust::RUST, arg_matches)) => app::rust::handle(arg_matches),
        Some(("mod", arg_matches)) => app::rust::mod_handle(arg_matches),
        Some(("app", arg_matches)) => app::rust::app_handle(arg_matches),
        Some(("deploy", arg_matches)) => app::deploy::handle(arg_matches),
        Some(("kill", arg_matches)) => app::sh::kill_handle(arg_matches),
        Some(("code", arg_matches)) => app::code::handle(arg_matches),
        Some(("docker", arg_matches)) => app::docker::handle(arg_matches),
        Some(("linux", arg_matches)) => app::linux::handle(arg_matches),
        Some(("sync", _arg_matches)) => app::linux::sync::run(),
        Some(("mega", arg_matches)) => app::megasync::handle(arg_matches),

        _ => println!("No Function Found"),
    }
}