//! Authentication and credential management
//!
//! Unified authentication manager that handles credentials using keyring or encrypted storage.

use crate::security::{
    audit::{AuditEntry, AuditEventType, AuditLogger},
    encryption,
    keyring::KeyringManager,
};
use crate::utils::error::{MultiGitError, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{debug, info};

/// Authentication backend
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthBackend {
    Keyring,
    EncryptedFile,
    Environment,
}

/// Authentication manager
pub struct AuthManager {
    keyring: Option<KeyringManager>,
    encrypted_store: Option<EncryptedCredentialStore>,
    audit_logger: Option<AuditLogger>,
    preferred_backend: AuthBackend,
}

impl AuthManager {
    /// Create a new auth manager
    pub fn new(preferred_backend: AuthBackend, enable_audit: bool) -> Self {
        let keyring = if matches!(preferred_backend, AuthBackend::Keyring) {
            Some(KeyringManager::new())
        } else {
            None
        };

        let audit_logger = if enable_audit {
            Some(AuditLogger::new(AuditLogger::default_path(), true))
        } else {
            None
        };

        Self {
            keyring,
            encrypted_store: None,
            audit_logger,
            preferred_backend,
        }
    }

    /// Store a credential
    pub fn store_credential(&self, provider: &str, username: &str, token: &str) -> Result<()> {
        info!("Storing credential for {}:{}", provider, username);

        let result = match self.preferred_backend {
            AuthBackend::Keyring => {
                if let Some(ref keyring) = self.keyring {
                    keyring.store_provider_token(provider, username, token)
                } else {
                    Err(MultiGitError::Other("Keyring not initialized".to_string()))
                }
            }
            AuthBackend::EncryptedFile => {
                if let Some(ref store) = self.encrypted_store {
                    store.store(provider, username, token)
                } else {
                    Err(MultiGitError::Other(
                        "Encrypted store not initialized".to_string(),
                    ))
                }
            }
            AuthBackend::Environment => Err(MultiGitError::Other(
                "Cannot store credentials in environment variables".to_string(),
            )),
        };

        // Log audit event
        if let Some(ref logger) = self.audit_logger {
            let entry = AuditEntry::new(
                AuditEventType::CredentialStore,
                format!("{}:{}", provider, username),
                result.is_ok(),
            );
            logger.log(entry);
        }

        result
    }

    /// Retrieve a credential
    pub fn retrieve_credential(&self, provider: &str, username: &str) -> Result<String> {
        debug!("Retrieving credential for {}:{}", provider, username);

        // Try environment variables first
        let env_var = format!("MULTIGIT_{}_TOKEN", provider.to_uppercase());
        if let Ok(token) = std::env::var(&env_var) {
            debug!("Found token in environment variable: {}", env_var);
            return Ok(token);
        }

        // Try preferred backend
        let result = match self.preferred_backend {
            AuthBackend::Keyring | AuthBackend::Environment => {
                if let Some(ref keyring) = self.keyring {
                    keyring.retrieve_provider_token(provider, username)
                } else {
                    Err(MultiGitError::Other("Keyring not initialized".to_string()))
                }
            }
            AuthBackend::EncryptedFile => {
                if let Some(ref store) = self.encrypted_store {
                    store.retrieve(provider, username)
                } else {
                    Err(MultiGitError::Other(
                        "Encrypted store not initialized".to_string(),
                    ))
                }
            }
        };

        // Log audit event
        if let Some(ref logger) = self.audit_logger {
            let entry = AuditEntry::new(
                AuditEventType::CredentialRetrieve,
                format!("{}:{}", provider, username),
                result.is_ok(),
            );
            logger.log(entry);
        }

        result
    }

    /// Remove a credential (alias for delete_credential)
    pub fn remove_credential(&self, provider: &str, username: &str) -> Result<()> {
        self.delete_credential(provider, username)
    }

    /// Delete a credential
    pub fn delete_credential(&self, provider: &str, username: &str) -> Result<()> {
        info!("Deleting credential for {}:{}", provider, username);

        let result = match self.preferred_backend {
            AuthBackend::Keyring => {
                if let Some(ref keyring) = self.keyring {
                    keyring.delete_provider_token(provider, username)
                } else {
                    Err(MultiGitError::Other("Keyring not initialized".to_string()))
                }
            }
            AuthBackend::EncryptedFile => {
                if let Some(ref store) = self.encrypted_store {
                    store.delete(provider, username)
                } else {
                    Err(MultiGitError::Other(
                        "Encrypted store not initialized".to_string(),
                    ))
                }
            }
            AuthBackend::Environment => Err(MultiGitError::Other(
                "Cannot delete environment variables".to_string(),
            )),
        };

        // Log audit event
        if let Some(ref logger) = self.audit_logger {
            let entry = AuditEntry::new(
                AuditEventType::CredentialDelete,
                format!("{}:{}", provider, username),
                result.is_ok(),
            );
            logger.log(entry);
        }

        result
    }

    /// Initialize encrypted file store with passphrase
    pub fn init_encrypted_store(&mut self, passphrase: String, path: PathBuf) {
        self.encrypted_store = Some(EncryptedCredentialStore::new(passphrase, path));
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new(AuthBackend::Keyring, false)
    }
}

/// Encrypted credential store (fallback when keyring unavailable)
struct EncryptedCredentialStore {
    passphrase: String,
    path: PathBuf,
}

impl EncryptedCredentialStore {
    fn new(passphrase: String, path: PathBuf) -> Self {
        Self { passphrase, path }
    }

    fn load_store(&self) -> Result<HashMap<String, String>> {
        if !self.path.exists() {
            return Ok(HashMap::new());
        }

        let encrypted = std::fs::read(&self.path)
            .map_err(|e| MultiGitError::Other(format!("Failed to read store: {}", e)))?;

        let decrypted = encryption::decrypt_with_passphrase(&encrypted, &self.passphrase)?;
        let json = String::from_utf8(decrypted)
            .map_err(|e| MultiGitError::Other(format!("Invalid UTF-8: {}", e)))?;

        serde_json::from_str(&json)
            .map_err(|e| MultiGitError::Other(format!("Invalid JSON: {}", e)))
    }

    fn save_store(&self, store: &HashMap<String, String>) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| MultiGitError::Other(format!("Failed to create dir: {}", e)))?;
        }

        let json = serde_json::to_string(store)
            .map_err(|e| MultiGitError::Other(format!("JSON serialize failed: {}", e)))?;

        let encrypted = encryption::encrypt_with_passphrase(json.as_bytes(), &self.passphrase)?;

        std::fs::write(&self.path, encrypted)
            .map_err(|e| MultiGitError::Other(format!("Failed to write store: {}", e)))?;

        Ok(())
    }

    fn store(&self, provider: &str, username: &str, token: &str) -> Result<()> {
        let mut store = self.load_store()?;
        let key = format!("{}:{}", provider, username);
        store.insert(key, token.to_string());
        self.save_store(&store)
    }

    fn retrieve(&self, provider: &str, username: &str) -> Result<String> {
        let store = self.load_store()?;
        let key = format!("{}:{}", provider, username);
        store
            .get(&key)
            .cloned()
            .ok_or_else(|| MultiGitError::Other(format!("Credential not found: {}", key)))
    }

    fn delete(&self, provider: &str, username: &str) -> Result<()> {
        let mut store = self.load_store()?;
        let key = format!("{}:{}", provider, username);
        store.remove(&key);
        self.save_store(&store)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_manager_creation() {
        let manager = AuthManager::new(AuthBackend::Keyring, false);
        assert!(manager.keyring.is_some());
    }

    #[test]
    fn test_env_var_credential() {
        std::env::set_var("MULTIGIT_GITHUB_TOKEN", "test_token_123");

        let manager = AuthManager::new(AuthBackend::Environment, false);
        let result = manager.retrieve_credential("github", "testuser");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_token_123");

        std::env::remove_var("MULTIGIT_GITHUB_TOKEN");
    }
}
