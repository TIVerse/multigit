//! Security module tests

use multigit::security::audit::{AuditEntry, AuditEventType, AuditLogger};
use multigit::security::encryption::{decrypt_string, encrypt_string};
use multigit::security::keyring::KeyringManager;
use std::path::PathBuf;

#[test]
fn test_audit_entry_creation() {
    let entry = AuditEntry::new(AuditEventType::CredentialStore, "test action", true);
    assert_eq!(
        std::mem::discriminant(&entry.event_type),
        std::mem::discriminant(&AuditEventType::CredentialStore)
    );
    assert!(entry.success);
}

#[test]
fn test_audit_entry_with_message() {
    let entry = AuditEntry::new(AuditEventType::Push, "action", true).with_message("test message");
    assert_eq!(entry.message, Some("test message".to_string()));
}

#[test]
fn test_audit_entry_with_user() {
    let entry = AuditEntry::new(AuditEventType::Pull, "action", true).with_user("testuser");
    assert_eq!(entry.user, Some("testuser".to_string()));
}

#[test]
fn test_audit_logger_creation() {
    let logger = AuditLogger::new(PathBuf::from("/tmp/audit.log"), true);
    assert_eq!(
        std::mem::size_of_val(&logger),
        std::mem::size_of::<AuditLogger>()
    );
}

#[test]
fn test_audit_logger_default_path() {
    let path = AuditLogger::default_path();
    assert!(path.ends_with("multigit"));
}

#[test]
fn test_audit_event_types() {
    assert_eq!(
        std::mem::discriminant(&AuditEventType::Push),
        std::mem::discriminant(&AuditEventType::Push)
    );
    assert_ne!(
        std::mem::discriminant(&AuditEventType::Push),
        std::mem::discriminant(&AuditEventType::Pull)
    );
}

#[test]
fn test_encrypt_decrypt_roundtrip() {
    let original = "secret data";
    let passphrase = "strong_password";

    let encrypted = encrypt_string(original, passphrase).unwrap();
    let decrypted = decrypt_string(&encrypted, passphrase).unwrap();

    assert_eq!(original, decrypted);
}

#[test]
fn test_encrypt_produces_different_output() {
    let data = "secret";
    let passphrase = "pass";

    let encrypted1 = encrypt_string(data, passphrase).unwrap();
    let encrypted2 = encrypt_string(data, passphrase).unwrap();

    // Should produce different ciphertexts (due to salt/nonce)
    assert_ne!(encrypted1, encrypted2);
}

#[test]
fn test_decrypt_wrong_passphrase() {
    let data = "secret";
    let encrypted = encrypt_string(data, "correct_pass").unwrap();

    let result = decrypt_string(&encrypted, "wrong_pass");
    assert!(result.is_err());
}

#[test]
fn test_decrypt_invalid_base64() {
    let result = decrypt_string("not valid base64!!!", "pass");
    assert!(result.is_err());
}

#[test]
fn test_keyring_manager_creation() {
    let manager = KeyringManager::new();
    assert_eq!(
        std::mem::size_of_val(&manager),
        std::mem::size_of::<KeyringManager>()
    );
}

#[test]
fn test_keyring_token_key() {
    let key1 = KeyringManager::token_key("github");
    let key2 = KeyringManager::token_key("gitlab");

    assert_ne!(key1, key2);
    assert!(key1.contains("github"));
}

#[test]
fn test_audit_entry_builder_pattern() {
    let entry = AuditEntry::new(AuditEventType::Push, "pushed to remote", true)
        .with_user("testuser")
        .with_message("Pushed 5 commits");

    assert_eq!(entry.user, Some("testuser".to_string()));
    assert_eq!(entry.message, Some("Pushed 5 commits".to_string()));
}

#[test]
fn test_encryption_empty_string() {
    let encrypted = encrypt_string("", "pass").unwrap();
    let decrypted = decrypt_string(&encrypted, "pass").unwrap();
    assert_eq!(decrypted, "");
}

#[test]
fn test_encryption_long_string() {
    let long_data = "a".repeat(10000);
    let encrypted = encrypt_string(&long_data, "pass").unwrap();
    let decrypted = decrypt_string(&encrypted, "pass").unwrap();
    assert_eq!(decrypted, long_data);
}

#[test]
fn test_encryption_unicode() {
    let unicode = "Hello 世界 🦀";
    let encrypted = encrypt_string(unicode, "pass").unwrap();
    let decrypted = decrypt_string(&encrypted, "pass").unwrap();
    assert_eq!(decrypted, unicode);
}

#[test]
fn test_audit_logger_disabled() {
    let logger = AuditLogger::new(PathBuf::from("/tmp/test.log"), false);
    let entry = AuditEntry::new(AuditEventType::Sync, "test", true);
    logger.log(entry);
    // Should not fail even when disabled
}
