use totp_lite::{totp_custom, Sha1};

/// Generate a 6-digit TOTP code from a base32-encoded secret
pub fn generate_totp(secret: &str) -> Result<String, String> {
    // Decode base32 secret
    let secret_bytes = base32::decode(base32::Alphabet::RFC4648 { padding: false }, secret)
        .ok_or_else(|| "Invalid base32 secret".to_string())?;

    // Get current Unix timestamp
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("Time error: {e}"))?
        .as_secs();

    // Generate TOTP (6 digits, 30 second window)
    let code = totp_custom::<Sha1>(30, 6, &secret_bytes, seconds);

    Ok(format!("{code:06}"))
}

/// Get seconds remaining in current TOTP window (0-29)
pub fn get_totp_remaining_seconds() -> u64 {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    30 - (seconds % 30)
}

/// Validate a TOTP secret (check if it's valid base32)
pub fn _validate_totp_secret(secret: &str) -> bool {
    base32::decode(base32::Alphabet::RFC4648 { padding: false }, secret).is_some()
}

/// Format TOTP code with space (123 456)
pub fn format_totp_code(code: &str) -> String {
    if code.len() == 6 {
        format!("{} {}", &code[..3], &code[3..])
    } else {
        code.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_totp_generation() {
        // RFC 6238 test vector
        let secret = "JBSWY3DPEHPK3PXP"; // "Hello!" in base32
        let result = generate_totp(secret);
        assert!(result.is_ok());
        let code = result.unwrap();
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_invalid_secret() {
        let result = generate_totp("INVALID!!!!");
        assert!(result.is_err());
    }

    #[test]
    fn test_remaining_seconds() {
        let remaining = get_totp_remaining_seconds();
        assert!(remaining > 0 && remaining <= 30);
    }

    #[test]
    fn test_format_code() {
        assert_eq!(format_totp_code("123456"), "123 456");
        assert_eq!(format_totp_code("12345"), "12345");
    }
}
