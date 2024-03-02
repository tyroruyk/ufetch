use std::io;
use std::path::Path;
use std::process::Command;

pub fn get_pkg() -> io::Result<String> {
    let mut result = String::new();

    if Path::new("/bin/dpkg").exists() {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("dpkg --get-selections | wc -l"))
            .output()?;

        result += &format!("{} (dpkg) ", String::from_utf8_lossy(&output.stdout).trim());
    }

    if Path::new("/bin/flatpak").exists() {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("flatpak list | wc -l"))
            .output()?;

        result += &format!(
            "{} (flatpak) ",
            String::from_utf8_lossy(&output.stdout).trim()
        );
    }

    if Path::new("/bin/pacman").exists() {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("pacman -Qq | wc -l"))
            .output()?;

        result += &format!(
            "{} (pacman) ",
            String::from_utf8_lossy(&output.stdout).trim()
        );
    }

    if Path::new("/var/lib/rpm").exists() {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("rpm -qa | wc -l"))
            .output()?;

        result += &format!("{} (rpm) ", String::from_utf8_lossy(&output.stdout).trim());
    }

    if Path::new("/bin/snap").exists() {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("snap list | wc -l"))
            .output()?;

        result += &format!("{} (snap) ", String::from_utf8_lossy(&output.stdout).trim());
    }

    Ok(result)
}
