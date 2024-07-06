
use console::{Key, Term};
use crate::styles::ApplyStyle;
use std::{io, process::Command as ShellCommand};
use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
};

pub fn exe (command: &str, noconfirm_flag: bool) {
    loop {
        if noconfirm_flag {
            run_shell_command(command);
            return;
        }

        println!("{}: {}\n", "Next command".blue(),  command.white());
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
                    println!("{}: {}", "Skipping command".yellow(), command.white());
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

pub fn print_cmd(command: &str){
    println!("{}: {} \n", "Command".blue(), command.white());
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
    print_cmd(command);
    ShellCommand::new("sh")
    .arg("-c")
    .arg(command)
    .status()
    .expect("Failed to execute command");
}
    