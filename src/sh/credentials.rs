
use std::io::{self, Write};

#[derive(Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,

}

impl  Credentials {
     /// Prompts the user to enter their username and password from the command line.
    ///
    /// # Errors
    /// This function will return an error if there is a failure in reading input from the user.
    ///
    /// - It can return `std::io::Error` if there is an I/O error.
    /// - It can return other errors that implement the `std::error::Error` trait.
    
    pub fn input() -> Result<Credentials, Box<dyn std::error::Error>>{
        let mut username = String::new();
        let mut password = String::new();

        print!("Enter username: ");
        io::stdout().flush()?; // Make sure the prompt is printed immediately
        io::stdin().read_line(&mut username)?;
        let username = username.trim().to_string(); // Trim to remove any newline characters
        
        print!("Enter password: "); // Prompt for password
        io::stdout().flush()?; // Make sure the prompt is printed immediately
        io::stdin().read_line(&mut password)?;
        let password = password.to_string(); // Trim to remove any newline characters
        
        Ok(Credentials { username, password })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;


    #[test]
    fn test_input_credentials() {
        let credentials = Credentials::input();
        println!("Credentials :{credentials:?}");
    }
}