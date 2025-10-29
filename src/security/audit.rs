//! Audit logging for sensitive operations
//!
//! Tracks security-sensitive operations for compliance and debugging.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use tracing::{debug, warn};

/// Audit event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    /// Credential storage event
    CredentialStore,
    /// Credential retrieval event
    CredentialRetrieve,
    /// Credential deletion event
    CredentialDelete,
    /// Remote addition event
    RemoteAdd,
    /// Remote removal event
    RemoteRemove,
    /// Push operation event
    Push,
    /// Pull operation event
    Pull,
    /// Sync operation event
    Sync,
}

/// An audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Timestamp when the event occurred
    pub timestamp: DateTime<Utc>,
    /// Type of audit event
    pub event_type: AuditEventType,
    /// User who performed the operation
    pub user: Option<String>,
    /// Resource affected by the operation
    pub resource: String,
    /// Whether the operation was successful
    pub success: bool,
    /// Optional message with additional context
    pub message: Option<String>,
}

impl AuditEntry {
    /// Create a new audit entry
    pub fn new(event_type: AuditEventType, resource: impl Into<String>, success: bool) -> Self {
        Self {
            timestamp: Utc::now(),
            event_type,
            user: None,
            resource: resource.into(),
            success,
            message: None,
        }
    }

    /// Add a message to the audit entry
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Add a user to the audit entry
    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

/// Audit logger
pub struct AuditLogger {
    log_path: PathBuf,
    enabled: bool,
}

impl AuditLogger {
    /// Create a new audit logger
    #[must_use]
    pub fn new(log_path: PathBuf, enabled: bool) -> Self {
        Self { log_path, enabled }
    }

    /// Get default audit log path
    #[must_use]
    pub fn default_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("multigit");
        path.push("audit.log");
        path
    }

    /// Log an audit entry
    pub fn log(&self, entry: AuditEntry) {
        if !self.enabled {
            return;
        }

        debug!("Audit: {:?}", entry);

        // Ensure parent directory exists
        if let Some(parent) = self.log_path.parent() {
            if !parent.exists() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    warn!("Failed to create audit log directory: {}", e);
                    return;
                }
            }
        }

        // Write to audit log file
        let mut file = match OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
        {
            Ok(f) => f,
            Err(e) => {
                warn!("Failed to open audit log: {}", e);
                return;
            }
        };

        let json = match serde_json::to_string(&entry) {
            Ok(j) => j,
            Err(e) => {
                warn!("Failed to serialize audit entry: {}", e);
                return;
            }
        };

        if let Err(e) = writeln!(file, "{json}") {
            warn!("Failed to write audit log: {}", e);
        }
    }

    /// Read audit entries from the log
    pub fn read_entries(&self) -> std::io::Result<Vec<AuditEntry>> {
        if !self.log_path.exists() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(&self.log_path)?;
        let entries: Vec<AuditEntry> = content
            .lines()
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();

        Ok(entries)
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new(Self::default_path(), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_audit_entry_creation() {
        let entry = AuditEntry::new(AuditEventType::CredentialStore, "github:testuser", true);

        assert!(entry.success);
        assert_eq!(entry.resource, "github:testuser");
    }

    #[test]
    fn test_audit_logger() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("audit.log");

        let logger = AuditLogger::new(log_path.clone(), true);

        let entry =
            AuditEntry::new(AuditEventType::Push, "origin", true).with_message("Pushed to GitHub");

        logger.log(entry);

        let entries = logger.read_entries().unwrap();
        assert_eq!(entries.len(), 1);
        assert!(entries[0].success);
    }
}
