use std::process::{Command, exit};
use std::io;

pub fn get_disk() -> io::Result<String> {
    // Run the df command and capture its output
    let output = Command::new("df").output();

    // Check if the command was successful
    match output {
        Ok(output) => {
            // Convert the output to a string
            let output_str = String::from_utf8_lossy(&output.stdout);

            // Extract the filesystem and usage information
            if let Some(line) = output_str.lines().nth(1) {
                // Split the line into columns
                let columns: Vec<&str> = line.split_whitespace().collect();

                // Check if the columns have the expected format
                if columns.len() >= 5 {
                    let size = columns[1];
                    let used = columns[2];
                    let usage_percentage = columns[4];

                    // Return disk usage information as a formatted string
                    return Ok(format!(
                        "{} of {} ({})",
                        size, used, usage_percentage
                    ));
                }
            }

            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Disk usage information not found",
            ))
        }
        Err(error) => {
            // Print the error and exit the program
            eprintln!("Failed to execute df command: {}", error);
            exit(1);
        }
    }
}