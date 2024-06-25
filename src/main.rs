
use clap::{Arg, Command};
use std::io::stdin;
use console::{Style, Key, Term};
trait ApplyStyle {
    fn green(self) -> String;
    fn cyan(self) -> String;
    fn blue(self) -> String;
    fn red(self) -> String;
    fn yellow(self) -> String;
}

impl ApplyStyle for &str {
    fn green(self) -> String {
        Style::new().green().apply_to(self).to_string()
    }
    fn cyan(self) -> String {
        Style::new().cyan().apply_to(self).to_string()
    }
    fn blue(self) -> String {
        Style::new().blue().apply_to(self).to_string()
    }
    fn red(self) -> String {
        Style::new().red().apply_to(self).to_string()
    }
    fn yellow(self) -> String {
        Style::new().yellow().apply_to(self).to_string()
    }

}
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
    let style = Style::new();
    println!("Command: {}", command);
    println!("noconfirm: {}", noconfirm_flag);
    loop {
        let term = Term::stdout();
        let mut user_input= Key::Unknown;
        if !noconfirm_flag {
            println!("Next Command: {}", command);
            println!();
            println!(
                "Press {}: execute command; {}: skip; {}: force next steps; {}: quit script.",
                "Enter".green(),
                "N".yellow(),
                "F".cyan(),
                "Q".red()
            );


            // stdin().read_line(&mut user_input).unwrap();
            user_input = term.read_key().unwrap();
            print!("Read char: {:?}", user_input);

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

        match user_input{
            Key::Char('\n') | Key::Enter => {
                // if !force_param {
                    // println!("Executing command:");
                // }
                println!("Command: {}", command);

                // let output = if result_flag {
                //     ProcessCommand::new("sh")
                //         .arg("-c")
                //         .arg(command)
                //         .output()
                //         .expect("Failed to execute command")
                // } else {
                //     ProcessCommand::new("sh")
                //         .arg("-c")
                //         .arg(command)
                //         .status()
                //         .expect("Failed to execute command");
                //     continue;
                // };

                // if result_flag {
                //     println!("{}", String::from_utf8_lossy(&output.stdout));
                // }
                break;
            }
            Key::Char('n') | Key::Char('N') => {
                println!("Skipping command: {}", command);
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

