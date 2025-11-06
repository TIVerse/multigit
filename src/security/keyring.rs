//! OS keyring integration for secure credential storage
//!
//! Provides a cross-platform interface to store credentials securely using the OS keyring.

use crate::utils::error::{MultiGitError, Result};
use keyring::Entry;
use tracing::{debug, info};

/// Service name for keyring entries
const SERVICE_NAME: &str = "multigit";

/// Keyring manager for secure credential storage
pub struct KeyringManager {
    service: String,
}

impl KeyringManager {
    /// Create a new keyring manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            service: SERVICE_NAME.to_string(),
        }
    }

    /// Create a keyring manager with a custom service name
    #[must_use]
    pub fn with_service(service: String) -> Self {
        Self { service }
    }

    /// Store a credential in the keyring
    pub fn store(&self, key: &str, value: &str) -> Result<()> {
        debug!("Storing credential for key: {}", key);

        let entry = Entry::new(&self.service, key)
            .map_err(|e| MultiGitError::Other(format!("Failed to create keyring entry: {e}")))?;

        entry
            .set_password(value)
            .map_err(|e| MultiGitError::Other(format!("Failed to store credential: {e}")))?;

        info!("Successfully stored credential for key: {}", key);
        Ok(())
    }

    /// Retrieve a credential from the keyring
    pub fn retrieve(&self, key: &str) -> Result<String> {
        debug!("Retrieving credential for key: {}", key);

        let entry = Entry::new(&self.service, key)
            .map_err(|e| MultiGitError::Other(format!("Failed to create keyring entry: {e}")))?;

        let password = entry
            .get_password()
            .map_err(|e| MultiGitError::Other(format!("Failed to retrieve credential: {e}")))?;

        debug!("Successfully retrieved credential for key: {}", key);
        Ok(password)
    }

    /// Delete a credential from the keyring
    pub fn delete(&self, key: &str) -> Result<()> {
        debug!("Deleting credential for key: {}", key);

        let entry = Entry::new(&self.service, key)
            .map_err(|e| MultiGitError::Other(format!("Failed to create keyring entry: {e}")))?;

        entry
            .delete_password()
            .map_err(|e| MultiGitError::Other(format!("Failed to delete credential: {e}")))?;

        info!("Successfully deleted credential for key: {}", key);
        Ok(())
    }

    /// Check if a credential exists in the keyring
    #[must_use]
    pub fn exists(&self, key: &str) -> bool {
        self.retrieve(key).is_ok()
    }

    /// Store a provider token (host-bound for security)
    pub fn store_provider_token(&self, provider: &str, host: &str, username: &str, token: &str) -> Result<()> {
        let key = format!("{provider}:{host}:{username}:token");
        self.store(&key, token)
    }

    /// Retrieve a provider token (with automatic migration from legacy keys)
    pub fn retrieve_provider_token(&self, provider: &str, host: &str, username: &str) -> Result<String> {
        let key = format!("{provider}:{host}:{username}:token");
        
        // Try new host-bound key first
        if let Ok(token) = self.retrieve(&key) {
            Ok(token)
        } else {
            // Try legacy key format for migration
            let legacy_key = format!("{provider}:{username}:token");
            match self.retrieve(&legacy_key) {
                Ok(token) => {
                    debug!("Migrating legacy credential key to host-bound format");
                    
                    // Store under new key
                    if let Err(e) = self.store(&key, &token) {
                        debug!("Failed to migrate credential: {e}");
                    } else {
                        // Delete old key (ignore errors)
                        let _ = self.delete(&legacy_key);
                        info!("Successfully migrated credential to host-bound key: {provider}:{host}:{username}");
                    }
                    
                    Ok(token)
                }
                Err(e) => Err(e),
            }
        }
    }

    /// Delete a provider token (tries both new and legacy formats)
    pub fn delete_provider_token(&self, provider: &str, host: &str, username: &str) -> Result<()> {
        let key = format!("{provider}:{host}:{username}:token");
        let result = self.delete(&key);
        
        // Also try to delete legacy key if it exists
        let legacy_key = format!("{provider}:{username}:token");
        let _ = self.delete(&legacy_key);
        
        result
    }
}

impl Default for KeyringManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyring_store_and_retrieve() {
        let manager = KeyringManager::new();
        let test_key = "test_key_12345";
        let test_value = "test_secret_value";

        // Store
        let store_result = manager.store(test_key, test_value);
        if store_result.is_err() {
            // Keyring might not be available in CI/test environments
            eprintln!("Keyring not available in test environment");
            return;
        }

        // Retrieve
        let retrieved = manager.retrieve(test_key).unwrap();
        assert_eq!(retrieved, test_value);

        // Cleanup
        let _ = manager.delete(test_key);
    }

    #[test]
    fn test_keyring_exists() {
        let manager = KeyringManager::new();
        let test_key = "test_key_exists_67890";

        // Initially should not exist
        assert!(!manager.exists(test_key));

        // Store a value
        if manager.store(test_key, "value").is_ok() {
            // Should now exist
            assert!(manager.exists(test_key));

            // Cleanup
            let _ = manager.delete(test_key);
        }
    }

    #[test]
    fn test_provider_token_methods() {
        let manager = KeyringManager::new();

        let provider = "github";
        let host = "github.com";
        let username = "testuser";
        let token = "ghp_test_token_123";

        // Store provider token with host binding
        if manager
            .store_provider_token(provider, host, username, token)
            .is_ok()
        {
            // Retrieve provider token
            let retrieved = manager.retrieve_provider_token(provider, host, username).unwrap();
            assert_eq!(retrieved, token);

            // Cleanup
            let _ = manager.delete_provider_token(provider, host, username);
        }
    }

    #[test]
    fn test_legacy_token_migration() {
        let manager = KeyringManager::new();

        let provider = "gitlab";
        let host = "gitlab.com";
        let username = "testuser";
        let token = "glpat_test_token_456";

        // Simulate legacy key (without host)
        let legacy_key = format!("{provider}:{username}:token");
        if manager.store(&legacy_key, token).is_ok() {
            // Retrieve should migrate automatically
            let retrieved = manager.retrieve_provider_token(provider, host, username);
            
            if let Ok(retrieved_token) = retrieved {
                assert_eq!(retrieved_token, token);
                
                // New key should exist after migration
                let new_key = format!("{provider}:{host}:{username}:token");
                assert!(manager.exists(&new_key));
                
                // Cleanup both keys
                let _ = manager.delete(&new_key);
                let _ = manager.delete(&legacy_key);
            }
        }
    }
}
