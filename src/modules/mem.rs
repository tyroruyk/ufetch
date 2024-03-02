use std::fs::File;
use std::io::{self, BufRead};

pub fn get_mem() -> io::Result<String> {
    // Open the /proc/meminfo file
    let file = File::open("/proc/meminfo")?;

    // Variables to store MemTotal and MemFree values
    let mut mem_total: u64 = 0;
    let mut mem_free: u64 = 0;

    // Read the file line by line
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            // Extract MemTotal and MemFree values
            if line.starts_with("MemTotal:") {
                mem_total = parse_memory_value(&line);
            } else if line.starts_with("MemFree:") {
                mem_free = parse_memory_value(&line);
            }
        }
    }

    // Calculate UsedMem (MemTotal - MemFree)
    let used_mem = mem_total.saturating_sub(mem_free);

    // Return UsedMem as a string
    Ok(format!(
        "{}MiB of {}MiB ({}%)",
        used_mem / 1024,
        mem_total / 1024,
        (used_mem * 100) / mem_total
    ))
}

fn parse_memory_value(line: &str) -> u64 {
    // Extract the numeric value from the line (ignoring non-numeric characters)
    line.split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}
