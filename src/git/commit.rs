use crate::prelude::*;
use rustyline::{history::FileHistory, Editor};
use std::process::{exit, Command};

pub fn run() {
    H1!("git add, commit and push");
    status();

    println!("Adding all files...");
    exe!("git add -v *"; true);

    let status = status();

    for line in status.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(f_type), Some(file)) = (parts.next(), parts.next()) {
            let message = match f_type {
                "M" | "MM" => "Modified",
                "AM" => "Added and modified",
                "A" => "Added",
                "D" => "Deleted",
                _ => {
                    eprintln!("Unknown type: {f_type}");
                    continue;
                }
            };

            println!("========================================================");
            let default_message = format!("{file} - {message} :");
            let mut rl = Editor::<(), FileHistory>::new().unwrap();
            // Read the line with the default message prepopulated
            let readline = rl.readline_with_initial(
                " * Text your message for the commit * \n",
                (&default_message, ""),
            );

            let message = match readline {
                Ok(input) => input.trim().to_string(),
                Err(err) => {
                    eprintln!("Error reading input: {err:?}");
                    default_message.to_string()
                }
            };

            h2!("Committing for {}:", file);
            exe!("git commit -m '{message}'"; true);
        }
    }
    h2!("Pushing");
    exe!("git push -v");
}

fn status() -> String {
    h2!("Git status");
    let output = Command::new("git")
        .arg("status")
        .arg("-s")
        .output()
        .unwrap_or_else(|e| {
            eprintln!("Failed to execute git command to check status: {e}");
            exit(1);
        });

    let git_status = String::from_utf8(output.stdout).unwrap_or_default();

    if git_status.trim().is_empty() {
        println!("Nothing to do. Exiting...");
        exit(0);
    }
    println!("{git_status}");
    git_status
}
