use crate::vault_ffi;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordStrength {
    pub score: i32,
    pub strength: String,
    pub percentage: i32,
    pub feedback: Vec<String>,
}

pub fn init_crypto() -> Result<(), String> {
    vault_ffi::init()
}

pub fn cleanup() {
    vault_ffi::cleanup();
}

pub fn gen_salt() -> String {
    if let Ok(salt_bytes) = vault_ffi::generate_salt() {
        hex::encode(salt_bytes)
    } else {
        let mut rng = rand::thread_rng();
        let salt_bytes: Vec<u8> = (0..vault_ffi::SALT_LENGTH).map(|_| rng.gen()).collect();
        hex::encode(salt_bytes)
    }
}

pub fn enc(data: &[u8], pwd: &str, salt_hex: &str) -> Result<Vec<u8>, String> {
    let salt = hex::decode(salt_hex).map_err(|_| "Invalid salt hex")?;
    vault_ffi::encrypt_data(data, pwd, &salt)
}

pub fn dec(data: &[u8], pwd: &str, salt_hex: &str) -> Result<Vec<u8>, String> {
    let salt = hex::decode(salt_hex).map_err(|_| "Invalid salt hex")?;
    vault_ffi::decrypt_data(data, pwd, &salt)
}

pub fn gen_pwd(len: usize) -> String {
    let chars =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{}|;:,.<>?";
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars.chars().nth(idx).unwrap()
        })
        .collect()
}

pub fn calc_pwd_strength(password: &str) -> PasswordStrength {
    let mut score = 0;
    let mut feedback = Vec::new();
    let length = password.len();

    if length >= 8 {
        score += 1;
    }
    if length >= 12 {
        score += 1;
    }
    if length >= 16 {
        score += 1;
    }
    if length < 8 {
        feedback.push("Use at least 8 characters".to_string());
    }

    let has_lower = password.chars().any(char::is_lowercase);
    let has_upper = password.chars().any(char::is_uppercase);
    let has_digit = password.chars().any(char::is_numeric);
    let has_special = password
        .chars()
        .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));

    if has_lower {
        score += 1;
    } else {
        feedback.push("Add lowercase letters".to_string());
    }

    if has_upper {
        score += 1;
    } else {
        feedback.push("Add uppercase letters".to_string());
    }

    if has_digit {
        score += 1;
    } else {
        feedback.push("Add numbers".to_string());
    }

    if has_special {
        score += 1;
    } else {
        feedback.push("Add special characters".to_string());
    }

    let lower_pwd = password.to_lowercase();
    let common_patterns = ["123", "abc", "password", "qwerty", "admin"];
    for pattern in &common_patterns {
        if lower_pwd.contains(pattern) {
            score -= 2;
            feedback.push("Avoid common patterns".to_string());
            break;
        }
    }

    if length >= 20 {
        score += 1;
    }

    if score < 0 {
        score = 0;
    }

    let (strength, percentage) = match score {
        0..=2 => ("Weak", 25),
        3..=4 => ("Fair", 50),
        5..=6 => ("Good", 75),
        _ => ("Strong", 100),
    };

    PasswordStrength {
        score,
        strength: strength.to_string(),
        percentage,
        feedback,
    }
}

#[allow(dead_code)]
pub fn secure_wipe(data: &mut [u8]) {
    vault_ffi::secure_zero(data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pwd_sw() {
        let result = calc_pwd_strength("abc");
        assert_eq!(result.strength, "Weak");
        assert!(result.score <= 2);
    }

    #[test]
    fn test_pwd_ss() {
        let result = calc_pwd_strength("MyP@ssw0rd!VeryStrong");
        assert_eq!(result.strength, "Strong");
        assert!(result.score >= 7);
    }

    #[test]
    fn test_pwd_cp() {
        let result = calc_pwd_strength("password123");
        assert!(result
            .feedback
            .iter()
            .any(|f| f.contains("common patterns")));
    }

    #[test]
    fn test_gpwdl() {
        let pwd = gen_pwd(16);
        assert_eq!(pwd.len(), 16);
    }

    #[test]
    fn test_gpwdv() {
        let pwd = gen_pwd(20);
        let has_lower = pwd.chars().any(char::is_lowercase);
        let has_upper = pwd.chars().any(char::is_uppercase);
        assert!(has_lower || has_upper);
    }

    #[test]
    fn test_sgen() {
        let salt1 = gen_salt();
        let salt2 = gen_salt();
        assert_ne!(salt1, salt2);
        assert_eq!(salt1.len(), 32); // 16 bytes = 32 hex chars
    }

    #[test]
    fn test_edr() {
        if init_crypto().is_err() {
            println!("Skipping crypto test - libsodium not available");
            return;
        }

        let plaintext = b"Hello, World! This is a test.";
        let password = "test_password_123";
        let salt = gen_salt();

        let encrypted = enc(plaintext, password, &salt).expect("Encryption failed");
        let decrypted = dec(&encrypted, password, &salt).expect("Decryption failed");

        assert_eq!(plaintext, &decrypted[..]);

        cleanup();
    }

    #[test]
    fn test_wpwdf() {
        if init_crypto().is_err() {
            println!("Skipping crypto test - libsodium not available");
            return;
        }

        let plaintext = b"Secret data";
        let password = "correct_password";
        let wrong_password = "wrong_password";
        let salt = gen_salt();

        let encrypted = enc(plaintext, password, &salt).expect("Encryption failed");
        let result = dec(&encrypted, wrong_password, &salt);

        assert!(result.is_err());

        cleanup();
    }

    #[test]
    fn test_sw() {
        let mut data = vec![1u8, 2, 3, 4, 5];
        secure_wipe(&mut data);
        assert_eq!(data, vec![0u8, 0, 0, 0, 0]);
    }
}
