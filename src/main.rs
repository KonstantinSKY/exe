
use clap::{Arg, Command};
use std::io::stdin;

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

    println!("Command: {}", command);
    println!("noconfirm: {}", noconfirm_flag)
    loop {
        let mut user_input = String::new();

        if !noconfirm_flag {
            println!("Next Command: {}", command);
            println!();
            println!(
                "Press Enter: execute command; N: skip; F: force next steps; Q: quit script."
            );

            // stdin().read_line(&mut user_input).unwrap();
            let user_input = read_single_char().unwrap();

            // // Move up three lines
            // execute!(
            //     io::stdout(),
            //     cursor::MoveUp(3),
            //     terminal::Clear(ClearType::CurrentLine),
            //     terminal::Clear(ClearType::CurrentLine),
            //     terminal::Clear(ClearType::CurrentLine),
            //     cursor::MoveUp(3)
            // )
            // .unwrap();

            // if user_input.eq_ignore_ascii_case("f") {
            //     force_param = true;
            //     println!("Force mode");
            //     continue;
            // }
        }

        // match user_input.to_lowercase().as_str() {
        //     "" => {
        //         if !force_param {
        //             println!("Executing command:");
        //         }
        //         println!("{}", command);

        //         let output = if result_flag {
        //             ProcessCommand::new("sh")
        //                 .arg("-c")
        //                 .arg(command)
        //                 .output()
        //                 .expect("Failed to execute command")
        //         } else {
        //             ProcessCommand::new("sh")
        //                 .arg("-c")
        //                 .arg(command)
        //                 .status()
        //                 .expect("Failed to execute command");
        //             continue;
        //         };

        //         if result_flag {
        //             println!("{}", String::from_utf8_lossy(&output.stdout));
        //         }
        //         break;
        //     }
        //     "n" => {
        //         println!("Skipping command: {}", command);
        //         break;
        //     }
        //     "q" => {
        //         println!("Quitting script.");
        //         std::process::exit(0);
        //     }
        //     _ => {
        //         println!("Invalid input.");
        //     }
        // }
    }
}

