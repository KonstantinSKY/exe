use std::process::{Command, Stdio};
use std::io::{self, BufRead, BufReader};
/// Finds and kills the process running on the specified port.
///
/// # Arguments
///
/// * `port` - A string slice that holds the port number.
///
/// # Errors
///
/// This function will return an `io::Result<()>` error if any of the following occurs:
/// - The `lsof` command fails to execute.
/// - The `kill` command fails to execute.
/// - There is an issue reading the output of the `lsof` command.
/// - No process is found running on the specified port.
///

pub fn run(port: &str) -> io::Result<()> {

    // Find the process ID using `lsof`
    let lsof_output = Command::new("lsof")
        .arg("-i")
        .arg(format!(":{port}"))
        .stdout(Stdio::piped())
        .output()?;

    if lsof_output.stdout.is_empty() {
        println!("No process found running on port {port}");
        return Ok(());
    }

    let reader = BufReader::new(lsof_output.stdout.as_slice());
    for line in reader.lines() {
        let line = line?;
        if line.contains("PID") {
            // Skip header line
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let Some(pid) = parts.get(1) {
            println!("Found process with PID: {pid}");

            // Kill the process using `kill`
            let kill_output = Command::new("kill")
                .arg("-9")
                .arg(pid)
                .output()?;

            if kill_output.status.success() {
                println!("Successfully killed process with PID: {pid}");
            } else {
                eprintln!("Failed to kill process with PID: {pid}");
            }
        }
    }

    Ok(())
}