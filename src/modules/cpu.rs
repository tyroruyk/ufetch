use std::fs::File;
use std::io::{self, BufRead};

pub fn get_cpu() -> io::Result<String> {
    // Open the /proc/cpuinfo file
    let file = File::open("/proc/cpuinfo")?; // /proc/cpuinfo contains the cpu informations

    // Read the file line by line
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            // Check for the "model name" line
            if line.starts_with("model name") {
                // Extract the model name
                let model_name = line.split(':').nth(1).map(|s| s.trim());
                return Ok(model_name.unwrap_or("N/A").to_string());
            }
        }
    }

    // If no "model name" line is found
    Ok("N/A".to_string())
}
