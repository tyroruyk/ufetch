use std::env;
use std::io;

pub fn get_de() -> io::Result<String> {
    // Attempt to retrieve the value of each environment variable in order
    if let Ok(de) = env::var("XDG_CURRENT_DESKTOP") {
        Ok(de)
    } else if let Ok(de) = env::var("DESKTOP_SESSION") {
        Ok(de)
    } else if let Ok(de) = env::var("XDG_SESSION_DESKTOP") {
        Ok(de)
    } else if let Ok(de) = env::var("CURRENT_DESKTOP") {
        Ok(de)
    } else if let Ok(de) = env::var("SESSION_DESKTOP") {
        Ok(de)
    } else {
        Ok("N/A".to_string())
    }
}
