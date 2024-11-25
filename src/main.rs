use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile;

fn get_vault_password() -> Result<String, String> {
    // First try environment variable
    if let Ok(password) = env::var("ANSIBLE_VAULT_PASSWORD") {
        return Ok(password);
    }

    // Then try password file from environment variable
    if let Ok(password_file) = env::var("ANSIBLE_VAULT_PASSWORD_FILE") {
        return fs::read_to_string(password_file)
            .map(|s| s.trim().to_string())
            .map_err(|e| format!("Failed to read vault password file: {}", e));
    }

    // Finally try default password file location
    let home = env::var("HOME").map_err(|_| "HOME environment variable not set".to_string())?;
    let default_password_file = Path::new(&home).join(".vault_pass");
    
    if default_password_file.exists() {
        return fs::read_to_string(default_password_file)
            .map(|s| s.trim().to_string())
            .map_err(|e| format!("Failed to read default vault password file: {}", e));
    }

    Err("No vault password found. Set ANSIBLE_VAULT_PASSWORD environment variable or create a password file".to_string())
}

fn decrypt_data(encrypted_data: &str) -> Result<String, String> {
    let vault_password = get_vault_password()?;
    
    // Create a temporary file for the vault password
    let temp_dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let pass_file = temp_dir.path().join("vault_pass");
    fs::write(&pass_file, &vault_password).map_err(|e| e.to_string())?;

    // Create a temporary file for the encrypted data
    let encrypted_file = temp_dir.path().join("encrypted.txt");
    fs::write(&encrypted_file, encrypted_data).map_err(|e| e.to_string())?;

    let output = Command::new("ansible-vault")
        .arg("decrypt")
        .arg("--vault-password-file")
        .arg(&pass_file)
        .arg(&encrypted_file)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        fs::read_to_string(&encrypted_file).map_err(|e| e.to_string())
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr).into_owned();
        Err(format!("Decryption failed: {}", err_msg))
    }
}

fn main() {
    // Ensure a filename is provided as an argument
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <vault_file.yml>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // Read the file into a vector of lines
    let lines = match fs::read_to_string(filename) {
        Ok(content) => content.lines().map(|s| s.to_string()).collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
            std::process::exit(1);
        }
    };

    // Regex to match lines with '!vault |', capturing the indentation
    let vault_re = Regex::new(r"^(\s*)(-?\s*.*?:?)?\s*!vault\s*\|").unwrap();

    let mut output_lines = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = &lines[i];

        if let Some(caps) = vault_re.captures(line) {
            let base_indent = caps.get(1).unwrap().as_str().len();
            let mut encrypted_data = String::new();

            output_lines.push(line.clone()); // Keep the '!vault |' line

            i += 1;
            // Collect encrypted data lines
            while i < lines.len() {
                let next_line = &lines[i];
                let next_line_indent = next_line.chars().take_while(|c| c.is_whitespace()).count();

                if next_line.trim().is_empty() {
                    output_lines.push(next_line.clone());
                    i += 1;
                    continue;
                }

                if next_line_indent > base_indent {
                    // Remove base indentation and collect encrypted data
                    let data_line = &next_line[base_indent..];
                    encrypted_data.push_str(data_line.trim_start());
                    encrypted_data.push('\n');
                    i += 1;
                } else {
                    break;
                }
            }

            // Decrypt the encrypted data
            let decrypted_data = match decrypt_data(&encrypted_data) {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("Failed to decrypt data: {}", err);
                    std::process::exit(1);
                }
            };

            // Indent decrypted data
            let decrypted_lines: Vec<&str> = decrypted_data.lines().collect();
            let encrypted_line_indent = " ".repeat(base_indent + 2); // Increase indent for decrypted lines

            output_lines.push(format!("{}|", encrypted_line_indent.trim_end()));

            for decrypted_line in decrypted_lines {
                let indented_line = format!("{}{}", encrypted_line_indent, decrypted_line);
                output_lines.push(indented_line);
            }
        } else {
            output_lines.push(line.clone());
            i += 1;
        }
    }

    // Output the result
    for line in output_lines {
        println!("{}", line);
    }
}
