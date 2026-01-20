//! Auth credential prompts
//!
//! Handles username and password collection with random generation option.

use anyhow::{Result, anyhow};
use console::{Term, style};
use dialoguer::{Confirm, Input, Password, Select};
use rand::Rng;
use rand::distr::Alphanumeric;

/// Handle Ctrl+C by restoring cursor and returning error
fn handle_interrupt() -> anyhow::Error {
    let _ = Term::stdout().show_cursor();
    anyhow!("Setup cancelled")
}

/// Validate username according to rules
fn validate_username(input: &str) -> Result<(), String> {
    if input.is_empty() {
        return Err("Username cannot be empty".to_string());
    }
    if input.len() < 3 {
        return Err("Username must be at least 3 characters".to_string());
    }
    if input.len() > 32 {
        return Err("Username must be at most 32 characters".to_string());
    }
    if !input.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Username must contain only letters, numbers, and underscores".to_string());
    }
    Ok(())
}

/// Generate a secure random password
fn generate_random_password() -> String {
    rand::rng()
        .sample_iter(Alphanumeric)
        .take(24)
        .map(char::from)
        .collect()
}

/// Prompt for authentication credentials
///
/// Offers choice between random generation and manual entry.
/// Returns (username, password) tuple.
pub fn prompt_auth(step: usize, total: usize) -> Result<(String, String)> {
    println!(
        "{} {}",
        style(format!("[{}/{}]", step, total)).dim(),
        style("Authentication").bold()
    );
    println!();

    loop {
        // Ask how user wants to set credentials
        let options = vec![
            "Generate secure random credentials",
            "Enter my own username and password",
        ];

        let selection = Select::new()
            .with_prompt("How would you like to set credentials?")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|_| handle_interrupt())?;

        match selection {
            0 => {
                // Random generation
                let password = generate_random_password();

                println!();
                println!("{}", style("Generated credentials:").green());
                println!("  Username: {}", style("admin").cyan());
                println!("  Password: {}", style(&password).cyan());
                println!();
                println!(
                    "{}",
                    style("Save these credentials securely - the password won't be shown again.")
                        .yellow()
                );
                println!();

                let use_these = Confirm::new()
                    .with_prompt("Use these credentials?")
                    .default(true)
                    .interact()
                    .map_err(|_| handle_interrupt())?;

                if use_these {
                    return Ok(("admin".to_string(), password));
                }
                // If not accepted, loop back to selection
                println!();
            }
            1 => {
                // Manual entry
                println!();

                let username: String = Input::new()
                    .with_prompt("Username")
                    .validate_with(|input: &String| validate_username(input))
                    .interact_text()
                    .map_err(|_| handle_interrupt())?;

                let password = Password::new()
                    .with_prompt("Password")
                    .with_confirmation("Confirm password", "Passwords do not match")
                    .interact()
                    .map_err(|_| handle_interrupt())?;

                if password.is_empty() {
                    println!("{}", style("Password cannot be empty").red());
                    println!();
                    continue;
                }

                return Ok((username, password));
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username_valid() {
        assert!(validate_username("admin").is_ok());
        assert!(validate_username("user_123").is_ok());
        assert!(validate_username("ABC").is_ok());
        assert!(validate_username("a_b_c_d_e_f_g_h_i_j_k_l_m_n_").is_ok()); // 32 chars
    }

    #[test]
    fn test_validate_username_empty() {
        assert!(validate_username("").is_err());
    }

    #[test]
    fn test_validate_username_too_short() {
        assert!(validate_username("ab").is_err());
    }

    #[test]
    fn test_validate_username_too_long() {
        let long = "a".repeat(33);
        assert!(validate_username(&long).is_err());
    }

    #[test]
    fn test_validate_username_invalid_chars() {
        assert!(validate_username("user@name").is_err());
        assert!(validate_username("user-name").is_err());
        assert!(validate_username("user name").is_err());
    }

    #[test]
    fn test_generate_random_password_length() {
        let password = generate_random_password();
        assert_eq!(password.len(), 24);
    }

    #[test]
    fn test_generate_random_password_alphanumeric() {
        let password = generate_random_password();
        assert!(password.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_generate_random_password_uniqueness() {
        // Generate multiple passwords and ensure they're different
        let p1 = generate_random_password();
        let p2 = generate_random_password();
        let p3 = generate_random_password();
        assert_ne!(p1, p2);
        assert_ne!(p2, p3);
        assert_ne!(p1, p3);
    }
}
