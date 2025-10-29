//! Encrypted file storage for credentials
//!
//! Provides age-based encryption as a fallback when OS keyring is unavailable.

use crate::utils::error::{MultiGitError, Result};
use age::secrecy::Secret;
use age::{Decryptor, Encryptor};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::io::{Read, Write};
use tracing::{debug, info};

/// Encrypt data using age encryption with a passphrase
pub fn encrypt_with_passphrase(data: &[u8], passphrase: &str) -> Result<Vec<u8>> {
    debug!("Encrypting data with passphrase");

    let encryptor = Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));

    let mut encrypted = Vec::new();
    let mut writer = encryptor
        .wrap_output(&mut encrypted)
        .map_err(|e| MultiGitError::Other(format!("Encryption failed: {e}")))?;

    writer
        .write_all(data)
        .map_err(|e| MultiGitError::Other(format!("Write failed: {e}")))?;

    writer
        .finish()
        .map_err(|e| MultiGitError::Other(format!("Encryption finish failed: {e}")))?;

    info!("Successfully encrypted data");
    Ok(encrypted)
}

/// Decrypt data using age encryption with a passphrase
pub fn decrypt_with_passphrase(encrypted_data: &[u8], passphrase: &str) -> Result<Vec<u8>> {
    debug!("Decrypting data with passphrase");

    let decryptor = match Decryptor::new(encrypted_data)
        .map_err(|e| MultiGitError::Other(format!("Decryption setup failed: {e}")))?
    {
        Decryptor::Passphrase(d) => d,
        _ => {
            return Err(MultiGitError::Other(
                "Unexpected decryptor type".to_string(),
            ))
        }
    };

    let mut decrypted = Vec::new();
    let mut reader = decryptor
        .decrypt(&Secret::new(passphrase.to_owned()), None)
        .map_err(|e| MultiGitError::Other(format!("Decryption failed: {e}")))?;

    reader
        .read_to_end(&mut decrypted)
        .map_err(|e| MultiGitError::Other(format!("Read failed: {e}")))?;

    info!("Successfully decrypted data");
    Ok(decrypted)
}

/// Encrypt a string and return base64-encoded result
pub fn encrypt_string(text: &str, passphrase: &str) -> Result<String> {
    let encrypted = encrypt_with_passphrase(text.as_bytes(), passphrase)?;
    Ok(STANDARD.encode(encrypted))
}

/// Decrypt a base64-encoded string
pub fn decrypt_string(encrypted_base64: &str, passphrase: &str) -> Result<String> {
    let encrypted = STANDARD.decode(encrypted_base64)
        .map_err(|e| MultiGitError::Other(format!("Base64 decode failed: {e}")))?;

    let decrypted = decrypt_with_passphrase(&encrypted, passphrase)?;

    String::from_utf8(decrypted)
        .map_err(|e| MultiGitError::Other(format!("UTF-8 decode failed: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let original = b"This is a secret message";
        let passphrase = "my_strong_passphrase_123";

        let encrypted = encrypt_with_passphrase(original, passphrase).unwrap();
        assert_ne!(encrypted, original);

        let decrypted = decrypt_with_passphrase(&encrypted, passphrase).unwrap();
        assert_eq!(decrypted, original);
    }

    #[test]
    fn test_encrypt_decrypt_string() {
        let original = "Secret token: ghp_123456789";
        let passphrase = "strong_password";

        let encrypted = encrypt_string(original, passphrase).unwrap();
        assert_ne!(encrypted, original);

        let decrypted = decrypt_string(&encrypted, passphrase).unwrap();
        assert_eq!(decrypted, original);
    }

    #[test]
    fn test_wrong_passphrase() {
        let original = b"Secret data";
        let passphrase = "correct_password";
        let wrong_passphrase = "wrong_password";

        let encrypted = encrypt_with_passphrase(original, passphrase).unwrap();

        let result = decrypt_with_passphrase(&encrypted, wrong_passphrase);
        assert!(result.is_err());
    }
}
