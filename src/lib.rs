#![warn(clippy::pedantic)]  

pub mod styles;

use console::{Key, Term};
use styles::ApplyStyle;
use std::{io, process::Command as ShellCommand};
use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
};


pub fn exec (command: &String, noconfirm_flag: bool) {
    styles::h1("Rust and ecosystem installation");
    loop {
        if noconfirm_flag {
            run_shell_command(command);
            return;
        }

        println!("Next Command: {}", command.white_bold());
        println!();
        println!(
            "Press {}: execute command; {}: skip; {}: force next steps; {}: quit script.",
            "Enter".green(),
            "N".yellow(),
            "F".cyan(),
            "Q".red()
        );
        
        let term = Term::stdout();
        let user_input = term.read_key().unwrap();


        clear_previous_lines(3);

        match user_input {
            Key::Char('\n') | Key::Enter => {
                run_shell_command(command); 
                break;
            }
            Key::Char('n') | Key::Char('N') => {
                println!("Skipping command: {}", command.white_bold());
                break;
            }
            Key::Char('q') | Key::Char('Q') => {
                println!("Quitting script.");
                std::process::exit(0);
            }
            _ => {
                println!("Invalid input.");
            }
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

fn run_shell_command(command: &String){
    println!("{}: {} \n", "Command".blue(), command.white_bold());

    ShellCommand::new("sh")
    .arg("-c")
    .arg(command)
    .status()
    .expect("Failed to execute command");
}
    
