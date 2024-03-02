use std::fs::File;
use std::io::{self, BufRead};

pub fn get_os() -> io::Result<String> {
    // Open the /etc/os-release file
    let file = File::open("/etc/os-release")?;

    // Read the file line by line
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            // Check for the "PRETTY_NAME" field
            if line.starts_with("PRETTY_NAME=") {
                // Extract the value of PRETTY_NAME
                let pretty_name = line.split('=').nth(1).map(|s| s.trim_matches('"'));
                return Ok(pretty_name.unwrap_or("N/A").to_string());
            }
        }
    }

    Ok("N/A".to_string())
}
