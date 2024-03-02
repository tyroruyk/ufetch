use std::env;
use std::io;

pub fn get_user() -> io::Result<String> {
    if let Ok(de) = env::var("USER") {
        Ok(de)
    } else {
        Ok("N/A".to_string())
    }
}
