use std::fs::read_to_string;
use std::io;

pub fn get_uptime() -> io::Result<String> {
    // Read the content of /proc/uptime into a string
    let uptime_str = read_to_string("/proc/uptime")?;

    // Parse the uptime value from the string
    let uptime_secs: f64 = uptime_str.trim().parse().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse uptime: {}", e))
    })?;

    // Calculate days, hours, and minutes
    let days = (uptime_secs / (24.0 * 3600.0)).floor() as u64;
    let hours = ((uptime_secs % (24.0 * 3600.0)) / 3600.0).floor() as u64;
    let minutes = ((uptime_secs % 3600.0) / 60.0).floor() as u64;

    // Format the result as "days, hours, minutes"
    Ok(format!("{}, {}, {}", days, hours, minutes))
}
