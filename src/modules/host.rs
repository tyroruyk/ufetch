use std::fs::read_to_string;
use std::io;
use std::path::Path;

pub fn get_host() -> io::Result<String> {
    // Construct the path to /proc/sys/kernel/hostname
    let path = Path::new("/proc/sys/kernel/hostname");

    // Read the content of the file into a string
    read_to_string(path).map(|s| s.trim().to_string())
}
