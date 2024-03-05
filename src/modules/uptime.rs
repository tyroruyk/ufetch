use std::process::{Command, exit};
use std::io;

pub fn get_uptime() -> io::Result<String> {
    // Run the uptime -p command and capture its output
    let output = Command::new("uptime").arg("-p").output();

    // Check if the command was successful
    match output {
        Ok(output) => {
            // Convert the output to a string
            let output_str = String::from_utf8_lossy(&output.stdout);

            // Extract the remaining text after 'up'
            if let Some(remaining_text) = output_str.strip_prefix("up") {
                return Ok(remaining_text.trim().to_string());
            }

            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Invalid 'uptime -p' output format",
            ))
        }
        Err(error) => {
            // Print the error and exit the program
            eprintln!("Failed to execute uptime command: {}", error);
            exit(1);
        }
    }
}