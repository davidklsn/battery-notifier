use std::process::Command;
use regex::Regex;

fn check_command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn main() {
    // Check if required commands are available
    if !check_command_exists("acpi") {
        eprintln!("Error: 'acpi' command not found. Please install acpi package.");
        return;
    }
    if !check_command_exists("notify-send") {
        eprintln!("Error: 'notify-send' command not found. Please install libnotify-bin package.");
        return;
    }

    // Execute the `acpi` command to get battery information
    let output = match Command::new("acpi")
        .arg("-b")
        .output() {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Failed to execute `acpi` command: {}", e);
                return;
            }
        };

    if !output.status.success() {
        eprintln!("acpi command failed: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }

    // Convert output to string
    let output_str = String::from_utf8_lossy(&output.stdout);
    if output_str.trim().is_empty() {
        eprintln!("No battery information found.");
        return;
    }

    // Define a regex to capture the percentage and time remaining
    let re = Regex::new(r"(?P<percentage>\d+)%.*(?P<time>\d{2}:\d{2}:\d{2})").unwrap();

    if let Some(captures) = re.captures(&output_str) {
        let percentage = captures.name("percentage").unwrap().as_str();
        let time_remaining = captures.name("time").unwrap().as_str();

        println!("Battery Percentage: {}%", percentage);
        println!("Time Remaining: {}", time_remaining);
    
    
        // Send a notification using `notify-send`
        match Command::new("notify-send")
            .arg("-u")
            .arg("normal")
            .arg(&format!("🔋 {}% ({})", percentage, time_remaining))
            .spawn() {
                Ok(_) => (),
                Err(e) => eprintln!("Failed to send notification: {}", e)
            }
    } else {
        eprintln!("Failed to parse battery info.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_command_exists() {
        // Test for a command that should definitely exist
        assert!(check_command_exists("ls"), "ls command should exist on Unix systems");
        // Test for a command that shouldn't exist
        assert!(!check_command_exists("nonexistentcommand123"), "nonexistent command should return false");
    }

    #[test]
    fn test_regex_pattern() {
        let re = Regex::new(r"(?P<percentage>\d+)%.*(?P<time>\d{2}:\d{2}:\d{2})").unwrap();
        
        // Test valid battery info format
        let sample_output = "Battery 0: Discharging, 75%, 02:30:45 remaining";
        let captures = re.captures(sample_output).unwrap();
        assert_eq!(captures.name("percentage").unwrap().as_str(), "75");
        assert_eq!(captures.name("time").unwrap().as_str(), "02:30:45");

        // Test different percentage and time
        let sample_output2 = "Battery 0: Charging, 25%, 01:15:30 remaining";
        let captures2 = re.captures(sample_output2).unwrap();
        assert_eq!(captures2.name("percentage").unwrap().as_str(), "25");
        assert_eq!(captures2.name("time").unwrap().as_str(), "01:15:30");
    }

    #[test]
    fn test_regex_invalid_input() {
        let re = Regex::new(r"(?P<percentage>\d+)%.*(?P<time>\d{2}:\d{2}:\d{2})").unwrap();
        
        // Test invalid format
        let invalid_output = "Invalid battery format";
        assert!(re.captures(invalid_output).is_none());

        // Test empty string
        let empty_output = "";
        assert!(re.captures(empty_output).is_none());
    }
}

