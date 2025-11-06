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
    /// OS keyring (secure)
    Keyring,
    /// Encrypted file storage
    EncryptedFile,
    /// Environment variables
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
    #[must_use]
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

    /// Store a credential (host-bound for security)
    pub fn store_credential(&self, provider: &str, host: &str, username: &str, token: &str) -> Result<()> {
        info!("Storing credential for {}:{}:{}", provider, host, username);

        let result = match self.preferred_backend {
            AuthBackend::Keyring => {
                if let Some(ref keyring) = self.keyring {
                    keyring.store_provider_token(provider, host, username, token)
                } else {
                    Err(MultiGitError::Other("Keyring not initialized".to_string()))
                }
            }
            AuthBackend::EncryptedFile => {
                if let Some(ref store) = self.encrypted_store {
                    store.store(provider, host, username, token)
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
                format!("{provider}:{host}:{username}"),
                result.is_ok(),
            );
            logger.log(entry);
        }

        result
    }

    /// Retrieve a credential (host-bound with automatic migration)
    pub fn retrieve_credential(&self, provider: &str, host: &str, username: &str, allow_env: bool) -> Result<String> {
        debug!("Retrieving credential for {}:{}:{}", provider, host, username);

        // Try environment variables if allowed
        if allow_env {
            let env_var = format!("MULTIGIT_{}_TOKEN", provider.to_uppercase());
            if let Ok(token) = std::env::var(&env_var) {
                info!("Using token from environment variable: {} (provider: {}, host: {})", env_var, provider, host);
                return Ok(token);
            }
        }

        // Try preferred backend
        let result = match self.preferred_backend {
            AuthBackend::Keyring | AuthBackend::Environment => {
                if let Some(ref keyring) = self.keyring {
                    keyring.retrieve_provider_token(provider, host, username)
                } else {
                    Err(MultiGitError::Other("Keyring not initialized".to_string()))
                }
            }
            AuthBackend::EncryptedFile => {
                if let Some(ref store) = self.encrypted_store {
                    store.retrieve(provider, host, username)
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
                format!("{provider}:{host}:{username}"),
                result.is_ok(),
            );
            logger.log(entry);
        }

        result
    }

    /// Remove a credential (alias for `delete_credential`)
    pub fn remove_credential(&self, provider: &str, host: &str, username: &str) -> Result<()> {
        self.delete_credential(provider, host, username)
    }

    /// Delete a credential (tries both host-bound and legacy keys)
    pub fn delete_credential(&self, provider: &str, host: &str, username: &str) -> Result<()> {
        info!("Deleting credential for {}:{}:{}", provider, host, username);

        let result = match self.preferred_backend {
            AuthBackend::Keyring => {
                if let Some(ref keyring) = self.keyring {
                    keyring.delete_provider_token(provider, host, username)
                } else {
                    Err(MultiGitError::Other("Keyring not initialized".to_string()))
                }
            }
            AuthBackend::EncryptedFile => {
                if let Some(ref store) = self.encrypted_store {
                    store.delete(provider, host, username)
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
                format!("{provider}:{host}:{username}"),
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
            .map_err(|e| MultiGitError::Other(format!("Failed to read store: {e}")))?;

        let decrypted = encryption::decrypt_with_passphrase(&encrypted, &self.passphrase)?;
        let json = String::from_utf8(decrypted)
            .map_err(|e| MultiGitError::Other(format!("Invalid UTF-8: {e}")))?;

        serde_json::from_str(&json).map_err(|e| MultiGitError::Other(format!("Invalid JSON: {e}")))
    }

    fn save_store(&self, store: &HashMap<String, String>) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| MultiGitError::Other(format!("Failed to create dir: {e}")))?;
        }

        let json = serde_json::to_string(store)
            .map_err(|e| MultiGitError::Other(format!("JSON serialize failed: {e}")))?;

        let encrypted = encryption::encrypt_with_passphrase(json.as_bytes(), &self.passphrase)?;

        std::fs::write(&self.path, encrypted)
            .map_err(|e| MultiGitError::Other(format!("Failed to write store: {e}")))?;

        Ok(())
    }

    fn store(&self, provider: &str, host: &str, username: &str, token: &str) -> Result<()> {
        let mut store = self.load_store()?;
        let key = format!("{provider}:{host}:{username}");
        store.insert(key, token.to_string());
        self.save_store(&store)
    }

    fn retrieve(&self, provider: &str, host: &str, username: &str) -> Result<String> {
        let store = self.load_store()?;
        let key = format!("{provider}:{host}:{username}");
        
        // Try new host-bound key first
        if let Some(token) = store.get(&key) {
            return Ok(token.clone());
        }
        
        // Try legacy key for migration
        let legacy_key = format!("{provider}:{username}");
        if let Some(token) = store.get(&legacy_key) {
            debug!("Found legacy credential, migrating to host-bound key");
            
            // Migrate to new key
            let mut new_store = store.clone();
            new_store.insert(key.clone(), token.clone());
            new_store.remove(&legacy_key);
            
            if let Err(e) = self.save_store(&new_store) {
                debug!("Failed to migrate credential: {}", e);
            } else {
                info!("Successfully migrated encrypted credential to host-bound key");
            }
            
            return Ok(token.clone());
        }
        
        Err(MultiGitError::Other(format!("Credential not found: {key}")))
    }

    fn delete(&self, provider: &str, host: &str, username: &str) -> Result<()> {
        let mut store = self.load_store()?;
        let key = format!("{provider}:{host}:{username}");
        store.remove(&key);
        
        // Also remove legacy key if exists
        let legacy_key = format!("{provider}:{username}");
        store.remove(&legacy_key);
        
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
        // Environment tokens must be explicitly allowed
        let result = manager.retrieve_credential("github", "github.com", "testuser", true);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_token_123");

        // Should not work when allow_env is false
        let result_no_env = manager.retrieve_credential("github", "github.com", "testuser", false);
        assert!(result_no_env.is_err());

        std::env::remove_var("MULTIGIT_GITHUB_TOKEN");
    }
}
