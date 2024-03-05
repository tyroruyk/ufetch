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

            // Extract the filesystem and space information
            if let Some(line) = output_str.lines().nth(1) {
                // Split the line into columns
                let columns: Vec<&str> = line.split_whitespace().collect();

                // Check if the columns have the expected format
                if columns.len() >= 4 {
                    let total_space = columns[1];
                    let used_space = columns[2];
                    let usage = columns[4];

                    // Parse the sizes in KiB and convert them to GiB
                    if let (Ok(total_kib), Ok(used_kib)) =
                        (total_space.parse::<f64>(), used_space.parse::<f64>())
                    {
                        let total_gib = total_kib / (1024.0 * 1024.0);
                        let used_gib = used_kib / (1024.0 * 1024.0);

                        // Return the storage information as a formatted string
                        return Ok(format!(
                            "{:.2}GiB of {:.2}GiB ({})",
                            used_gib, total_gib, usage
                        ));
                    }
                }
            }

            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Storage information not found",
            ))
        }
        Err(error) => {
            // Print the error and exit the program
            eprintln!("Failed to execute df command: {}", error);
            exit(1);
        }
    }
}
