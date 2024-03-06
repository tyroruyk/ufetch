use std::fs::read_to_string;
use std::io;
use std::path::Path;

pub fn get_machine() -> io::Result<String> {
    // Construct the path to /proc/sys/kernel/hostname
    let name = read_to_string(Path::new("/sys/devices/virtual/dmi/id/product_name"))?;
    let version = read_to_string(Path::new("/sys/devices/virtual/dmi/id/product_version"))?;

    Ok(format!("{} {}", name.trim().to_string(), version.trim().to_string()))
}
