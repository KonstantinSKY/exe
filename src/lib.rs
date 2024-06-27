#![warn(clippy::pedantic)]  

pub mod styles;
// pub mod insset;
pub mod rust;
pub mod sh;

use console::{Key, Term};
use styles::ApplyStyle;
use std::{io, process::Command as ShellCommand};
use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
};
use clap::Arg;

#[must_use] 
pub fn arg_no_confirm() -> Arg {
    Arg::new("noconfirm")
    .help("Skip confirmation flag")
    .short('n')
    .long("noconfirm")
    .action(clap::ArgAction::SetTrue)
}

pub fn exec (command: &str, noconfirm_flag: bool) {
    loop {
        if noconfirm_flag {
            run_shell_command(command);
            return;
        }

        println!("Next Command: {}", command.white_bold());
        println!();
        println!(
            "Press {}: execute command; {}: skip; {} quit script.",
            "Enter".green(),
            "N".yellow(),
            // "F".cyan(),
            "Q".red()
        );
        
        if let Ok(user_input) = Term::stdout().read_key() {
            clear_previous_lines(3);

            match user_input {
                Key::Char('\n') | Key::Enter => {
                    run_shell_command(command); 
                    break;
                }
                Key::Char('n' | 'N')  => {
                    println!("{}: {}", "Skipping command".yellow(), command.white_bold());
                    break;
                }
                Key::Char('q' | 'Q')  => {
                    println!("Quitting script.");
                    std::process::exit(0);
                }
                _ => {
                    println!("Invalid input. ");
                }
            }
        } else {
            eprintln!("Failed to read input.");
        }
    }
}


fn clear_previous_lines(lines: u16) {

    // Clear each of those lines
    for _ in 0..lines {
        execute!(
            io::stdout(),
            cursor::MoveUp(1),
            terminal::Clear(ClearType::CurrentLine),
        )
        .unwrap();
    }
}

fn run_shell_command(command: &str){
    println!("{}: {} \n", "Command".blue(), command.white_bold());

    ShellCommand::new("sh")
    .arg("-c")
    .arg(command)
    .status()
    .expect("Failed to execute command");
}
    
