use crate::prelude::*;
use console::{Key, Term};
use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};
use std::{io, process::Command as ShellCommand};

#[macro_export]
macro_rules! run {
    // Pattern for a function with no arguments, no message, and no confirmation flag
    ($func:expr) => {{
        $crate::sh::exec::run_fn($func, "", false);
    }};
    // Pattern for a function with a message but no confirmation flag
    ($func:expr, $msg:expr) => {{
        $crate::sh::exec::run_fn($func, $msg, false);
    }};
    // Pattern for a function with a confirmation flag but no message
    ($func:expr, $confirm:expr) => {{
        $crate::sh::exec::run_fn($func, "", $confirm);
    }};
    // Pattern for a function with both a message and a confirmation flag
    ($func:expr, $msg:expr, $confirm:expr) => {{
        $crate::sh::exec::run_fn($func, $msg, $confirm);
    }};
}

#[macro_export]
macro_rules! exe {
    ($command:literal $(, $arg:expr)*) => {
        $crate::sh::exec::exe(&format!($command $(, $arg)*), false);
    };
    // Pattern for interpolated command with the no_confirm flag
    ($command:literal $(, $arg:expr)*; $noconfirm_flag:expr) => {
        $crate::sh::exec::exe(&format!($command $(, $arg)*), $noconfirm_flag);
    };
}

pub fn run_fn<F>(function: F, message: &str, noconfirm_flag: bool)
where
    F: Fn(),
{
    loop {
        if noconfirm_flag {
            function();
            return;
        }
        println!("{}: {}\n", "Next Function".dark_blue(), message.white());
        println!(
            "Press {}: to Run Function; {}: skip; {} quit script.",
            "Enter".green(),
            "N".yellow(),
            // "F".cyan(),
            "Q".red()
        );
        match select_input() {
            Some(true) => {
                function();
                break;
            }
            Some(false) => {
                println!("{}: {}", "Skipping Function".yellow(), message.white());
                break;
            }
            None => {}
        }
    }
}

pub fn exe(command: &str, noconfirm_flag: bool) {
    loop {
        if noconfirm_flag {
            run_shell_command(command);
            return;
        }

        println!("{}: {}\n", "Next command".blue(), command.white());
        println!(
            "Press {}: execute command; {}: skip; {} quit script.",
            "Enter".green(),
            "N".yellow(),
            // "F".cyan(),
            "Q".red()
        );
        match select_input() {
            Some(true) => {
                run_shell_command(command);
                break;
            }
            Some(false) => {
                println!("{}: {}", "Skipping command".yellow(), command.white());
                break;
            }
            None => {}
        }
    }
}

fn select_input() -> Option<bool> {
    if let Ok(user_input) = Term::stdout().read_key() {
        clear_previous_lines(3);

        match user_input {
            Key::Char('\n') | Key::Enter => Some(true),
            Key::Char('n' | 'N') => Some(false),
            Key::Char('q' | 'Q') => {
                println!("Quitting program...");
                std::process::exit(0);
            }
            _ => {
                println!("Invalid input. ");
                None
            }
        }
    } else {
        eprintln!("Failed to read input.");
        None
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

fn run_shell_command(command: &str) {
    cmd!("{command}");
    ShellCommand::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .expect("Failed to execute command");
}
