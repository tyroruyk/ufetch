use std::process::{Command, exit};
use std::io;

pub fn get_gpu() -> io::Result<String> {
    // Run the lspci command and capture its output
    let output = Command::new("lspci").output();

    // Check if the command was successful
    match output {
        Ok(output) => {
            // Convert the output to a string
            let output_str = String::from_utf8_lossy(&output.stdout);

            // Search for GPU-related information in the lspci output
            if let Some(line) = output_str.lines().find(|line| line.to_lowercase().contains("vga") || line.to_lowercase().contains("3d")) {
                // Extract the GPU model information
                let gpu_model = line.split(':').nth(2).map(|s| s.trim());

                // Return the GPU model as a string
                return Ok(gpu_model.unwrap_or("N/A").to_string());
            }

            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "GPU information not found",
            ))
        }
        Err(error) => {
            // Print the error and exit the program
            eprintln!("Failed to execute lspci command: {}", error);
            exit(1);
        }
    }
}
