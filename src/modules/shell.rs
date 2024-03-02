use std::env;
use std::io;

pub fn get_shell() -> io::Result<String> {
    if let Ok(de) = env::var("SHELL") {
        Ok(de)
    } else {
        Ok("N/A".to_string())
    }
}
