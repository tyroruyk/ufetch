use std::process::{Command, exit};
use std::io;

pub fn get_kernel() -> io::Result<String> {
    // Run the uname -r command and capture its output
    let output = Command::new("uname").arg("-r").output();

    // Check if the command was successful
    match output {
        Ok(output) => {
            if output.status.success() {
                // Convert the output to a string
                let kernel_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                return Ok(kernel_version);
            } else {
                // Print the error code and exit the program
                eprintln!("Command failed with error code: {}", output.status);
                exit(1);
            }
        }
        Err(error) => {
            // Print the error and exit the program
            eprintln!("Failed to execute uname -r command: {}", error);
            exit(1);
        }
    }
}
