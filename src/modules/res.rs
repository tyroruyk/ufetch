use std::process::{Command, exit};
use std::io;

pub fn get_res() -> io::Result<String> {
    // Run the xrandr command and capture its output
    let output = Command::new("xrandr").output();

    // Check if the command was successful
    match output {
        Ok(output) => {
            // Convert the output to a string
            let output_str = String::from_utf8_lossy(&output.stdout);

            // Find the line containing the primary display resolution
            if let Some(line) = output_str.lines().find(|line| line.contains("*+")) {
                // Extract the resolution from the line
                if let Some(resolution) = line.split_whitespace().find(|&word| word.contains('x')) {
                    return Ok(resolution.to_string());
                }
            }

            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Primary display resolution not found",
            ))
        }
        Err(error) => {
            // Print the error and exit the program
            eprintln!("Failed to execute xrandr command: {}", error);
            exit(1);
        }
    }
}
