use ansible_vault as vault;
use anyhow::{anyhow, bail, Result};
use clap::Parser;
use is_executable::IsExecutable;
use log::{debug, error};
use once_cell::sync::Lazy;
use regex::Regex;
use std::env;
use std::fs::{self, File};
use std::io::{stdin, BufRead, BufReader, Cursor, Error};
use std::path::{Path, PathBuf};
use std::process;

static VAULT_PASSWORDS: Lazy<Vec<String>> = Lazy::new(|| {
    match get_vault_passwords() {
        Ok(passwords) => passwords,
        Err(e) => {
            error!("Error: {e}");
            process::exit(1);
        }
    }
});

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// YAML file with vaulted values
    file: Option<PathBuf>,
}

fn get_password_from_file(path: PathBuf) -> Result<String> {
    if path.is_executable() {
        debug!("Attempting to execute file: {}", path.display());
        let res = process::Command::new(path).output()?;
        if !res.status.success() {
            for err_line in res.stderr.lines() {
                error!("{}", err_line?)
            }
            bail!("Failed to execute vault password file: {}", res.status)
        }
        match res.stdout.lines().next() {
            None => bail!("Vault password is empty"),
            Some(password) => password
                .or_else(|e| Err(anyhow!("Failed to read vault password: {e}")))
        }
    } else {
        debug!("Attempting to read file: {}", path.display());
        fs::read_to_string(path)
            .or_else(|e| Err(anyhow!("Failed to read vault password file: {e}")))
    }
}

fn get_vault_passwords() -> Result<Vec<String>> {
    let mut passwords = Vec::new();

    // TODO: Read ansible.cfg from ANSIBLE_CONFIG or default one.

    // First try environment variable
    if let Ok(password) = env::var("ANSIBLE_VAULT_PASSWORD") {
        debug!("Attempting to get vault password from environment variable ANSIBLE_VAULT_PASSWORD");
        passwords.push(password);
    } else {
        debug!("ANSIBLE_VAULT_PASSWORD variable not found");
    }

    // Then try password file from environment variable
    let password_file = match env::var("ANSIBLE_VAULT_PASSWORD_FILE") {
        Ok(password_file) => {
            debug!("Attempting to get vault password from environment variable ANSIBLE_VAULT_PASSWORD_FILE");
            Some(password_file)
        }
        Err(_) => {
            debug!("ANSIBLE_VAULT_PASSWORD_FILE variable not found");
            // Try default password file location
            if let Ok(home) = env::var("HOME") {
                let default_password_file = Path::new(&home).join(".vault_pass");
                if default_password_file.exists() {
                    debug!("Attempting to get vault password from default location ~/.vault_pass");
                    if let Some(password_file) = default_password_file.to_str() {
                        Some(password_file.to_string())
                    } else {
                        None
                    }
                } else {
                    debug!("Default password file not found");
                    None
                }
            } else {
                debug!("HOME environment variable not set");
                None
            }
        }
    };
    if let Some(password_file) = password_file {
        let password = get_password_from_file(password_file.into())?;
        passwords.push(password);
    }

    // Finally try password files from identity list
    if let Ok(identity_list) = env::var("ANSIBLE_VAULT_IDENTITY_LIST") {
        debug!("Attempting to get vault passwords from environment variable ANSIBLE_VAULT_IDENTITY_LIST");
        for identity in identity_list.split(",") {
            if let Some((identity, password_file)) = identity.trim().split_once("@") {
                if identity.is_empty() { continue; }
                if let Ok(password) = get_password_from_file(password_file.into()) {
                    passwords.push(password);
                }
            }
        }
    } else {
        debug!("ANSIBLE_VAULT_IDENTITY_LIST variable not found");
    }

    if passwords.is_empty() {
        bail!("No vault password found. Set ANSIBLE_VAULT_PASSWORD environment variable or create a password file")
    }
    Ok(passwords)
}

fn decrypt_data(encrypted_data: &str) -> Result<String> {
    for vault_password in VAULT_PASSWORDS.iter() {
        // Use ansible-vault crate to decrypt the data
        let cursor = Cursor::new(encrypted_data);
        if let Ok(decrypted_bytes) = vault::decrypt_vault(cursor, vault_password) {
            return Ok(String::from_utf8(decrypted_bytes)?);
        }
    }
    bail!("Decryption failed: No matching vault password found")
}

fn read_file_or_stdout(path: Option<PathBuf>) -> Result<Vec<String>> {
    let file: Box<dyn BufRead> = match path {
        None => Box::new(BufReader::new(stdin())),
        Some(path) => Box::new(BufReader::new(File::open(path)?))
    };
    let data = file
        .lines()
        .collect::<Result<Vec<String>, Error>>()?;
    Ok(data)
}

fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default()
            .filter_or("LOG_LEVEL", "info")
            .write_style_or("LOG_STYLE", "always")
    );

    let args = Args::parse();
    let lines = match read_file_or_stdout(args.file) {
        Ok(lines) => lines,
        Err(err) => {
            error!("{err}");
            process::exit(1);
        }
    };

    // Regex to match lines with '!vault |', capturing the indentation
    let vault_re = Regex::new(r"^((\s*)-?\s*.*?:?)\s*!vault\s*\|")?;

    let mut output_lines = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = &lines[i];

        if let Some(caps) = vault_re.captures(line) {
            let pre_vault_text = caps.get(1).unwrap().as_str();
            let base_indent = caps.get(2).unwrap().as_str().len();
            let mut encrypted_data = String::new();

            i += 1;
            // Collect encrypted data lines
            while i < lines.len() {
                let next_line = &lines[i];
                let next_line_indent = next_line.chars().take_while(|c| c.is_whitespace()).count();
                // TODO: count tabs correctly. It may require scanning the file for first indentation
                //       to find out how many space symbols are used for one level of indentation.
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
                Ok(decrypted_data) => decrypted_data,
                Err(err) => {
                    error!("{err}");
                    process::exit(1);
                }
            };

            // Indent decrypted data
            let decrypted_lines: Vec<&str> = decrypted_data.lines().collect();
            match decrypted_lines.len() {
                0 => {
                    output_lines.push(format!("{pre_vault_text} ''"));
                }
                1 => {
                    let decrypted_line = decrypted_lines[0];
                    output_lines.push(format!("{pre_vault_text} {decrypted_line}"));
                }
                _ => {
                    // TODO: use the same indentation as the original file
                    let encrypted_line_indent = " ".repeat(base_indent + 2); // Increase indent for decrypted lines

                    output_lines.push(format!("{pre_vault_text} |"));
                    for decrypted_line in decrypted_lines {
                        let indented_line = format!("{encrypted_line_indent}{decrypted_line}");
                        output_lines.push(indented_line);
                    }
                }
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
    Ok(())
}
