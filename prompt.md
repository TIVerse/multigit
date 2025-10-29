# MultiGit Project Generation Prompt

> **Project Repository**: https://github.com/TIVerse/multigit  
> **Branch**: `master`  
> **Mission**: Build a production-ready, cross-platform Git multi-remote synchronization tool in Rust

---

## 🎭 Role & Context

You are an experienced **senior Rust systems engineer** with 10+ years building production CLI tools, distributed systems, and developer tooling. You have deep expertise in:
- Git internals and libgit2 bindings
- Async Rust programming with tokio
- Cross-platform development (Linux, macOS, Windows)
- RESTful API client development
- Security-first credential management
- TUI development with ratatui
- Performance optimization and concurrency

You write **idiomatic, production-grade Rust code** that feels natural, well-commented, and follows community best practices. Your code includes realistic edge cases, proper error messages, and thoughtful architecture decisions that a human would make after years of experience.

---

## 🎯 Project Overview

**MultiGit** is a Rust-based CLI tool that manages a single local Git repository synchronized across multiple remote Git hosting platforms (GitHub, GitLab, Bitbucket, Codeberg, Gitea, Forgejo) simultaneously.

### Core Value Proposition
- **"One repository. Infinite destinations."**
- Push, pull, sync, and mirror code across platforms with a single command
- Intelligent conflict resolution and automated synchronization
- Security-first credential management with OS keyring integration
- Daemon mode for background automation and scheduled syncs
- Rich CLI/TUI interfaces with real-time progress feedback

### Key Design Principles (from `.windsurf/rules`)
1. Cross-platform Git sync tool with multi-service support
2. Intelligent conflict detection without data loss
3. CLI, TUI, and optional GUI frontends
4. Rust-based with async network handling
5. Secure credential management (keyrings + SSH)
6. Granular sync configuration (per-branch, per-remote)
7. Offline-first with local queueing and background daemon
8. Webhook/cron automation support
9. Modular architecture (core engine + platform adapters + frontends)
10. Human-readable logs with modern progress indicators
11. Reliability and minimal configuration
12. Idiomatic Rust patterns with extensive tests

---

## 📋 Complete Project Structure

```
multigit/
├── Cargo.toml
├── README.md
├── LICENSE
├── .gitignore
├── .github/workflows/
│   ├── test.yml
│   ├── release.yml
│   └── coverage.yml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── init.rs
│   │   │   ├── remote.rs
│   │   │   ├── push.rs
│   │   │   ├── pull.rs
│   │   │   ├── sync.rs
│   │   │   ├── status.rs
│   │   │   ├── conflict.rs
│   │   │   ├── daemon.rs
│   │   │   └── doctor.rs
│   │   ├── parser.rs
│   │   └── interactive.rs
│   ├── core/
│   │   ├── mod.rs
│   │   ├── config.rs
│   │   ├── auth.rs
│   │   ├── sync_manager.rs
│   │   ├── conflict_resolver.rs
│   │   └── health_checker.rs
│   ├── providers/
│   │   ├── mod.rs
│   │   ├── traits.rs
│   │   ├── github.rs
│   │   ├── gitlab.rs
│   │   ├── bitbucket.rs
│   │   ├── codeberg.rs
│   │   └── gitea.rs
│   ├── git/
│   │   ├── mod.rs
│   │   ├── operations.rs
│   │   ├── remote.rs
│   │   └── branch.rs
│   ├── daemon/
│   │   ├── mod.rs
│   │   ├── service.rs
│   │   └── scheduler.rs
│   ├── security/
│   │   ├── mod.rs
│   │   ├── keyring.rs
│   │   ├── encryption.rs
│   │   └── audit.rs
│   ├── api/
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   ├── rate_limiter.rs
│   │   └── retry.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── repository.rs
│   │   ├── remote.rs
│   │   ├── config.rs
│   │   └── sync_state.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── tui.rs
│   │   ├── progress.rs
│   │   └── formatter.rs
│   └── utils/
│       ├── mod.rs
│       ├── logger.rs
│       ├── error.rs
│       └── validation.rs
├── tests/
│   ├── integration/
│   ├── unit/
│   └── fixtures/
├── benches/
├── examples/
├── docs/
└── scripts/
```

---

## 🔨 Human-Like Code Patterns

### ✅ DO Write Code Like This:

**Natural variable naming:**
```rust
// Good - reads naturally
let remote_connection_status = provider.test_connection().await?;
let configured_remotes = config.get_enabled_remotes();

// Avoid - too terse
let rcs = p.tc().await?;
```

**Thoughtful comments:**
```rust
// We need to verify the remote exists before attempting the push.
// This saves a network round-trip and provides clearer error messages.
if !self.remote_exists(&remote_name)? {
    return Err(MultiGitError::RemoteNotFound(remote_name.clone()));
}

// TODO(v0.5): Add support for custom merge strategies
// For now, we default to fast-forward only to prevent accidental data loss
let merge_strategy = MergeStrategy::FastForwardOnly;
```

**Helpful error messages:**
```rust
format!(
    "Failed to push to {}: {}.\n\nPossible causes:\n\
     - Invalid or expired token (run: multigit remote update {})\n\
     - Network connectivity issues\n\
     - Repository doesn't exist on {}\n\n\
     Run 'multigit remote test {}' to diagnose.",
    provider, err, provider, provider, provider
)
```

**Realistic function sizes (50-100 lines for workflows is OK):**
```rust
pub async fn sync_repository(&self) -> Result<SyncResult> {
    // 1. Fetch from all remotes
    tracing::info!("Fetching from all remotes...");
    let fetch_results = self.fetch_all().await?;
    
    // 2. Analyze state
    tracing::debug!("Analyzing repository state...");
    let state = self.analyze_sync_state(&fetch_results)?;
    
    // 3. Detect conflicts
    if state.has_conflicts() {
        return self.handle_conflicts(state).await;
    }
    
    // 4. Execute sync
    // ... continue with implementation
}
```

### ❌ AVOID These AI-Like Patterns:

- Perfect code symmetry everywhere
- Zero TODO comments or technical debt
- Overly generic abstractions without clear benefit
- Comments that just repeat the code
- Excessive use of Result::map chains
- Every function under 10 lines (unrealistic)

---

## 📦 Cargo.toml Dependencies

```toml
[package]
name = "multigit"
version = "0.1.0"
edition = "2021"
authors = ["TIVerse Team"]
description = "Universal Git multi-remote automation tool"
license = "MIT"
repository = "https://github.com/TIVerse/multigit"
keywords = ["git", "cli", "sync", "multi-remote"]
categories = ["command-line-utilities", "development-tools"]

[dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }
futures = "0.3"
async-trait = "0.1"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Git operations
git2 = { version = "0.18", features = ["vendored-libgit2"] }

# CLI
clap = { version = "4.4", features = ["derive", "cargo"] }
clap_complete = "4.4"
dialoguer = { version = "0.11", features = ["fuzzy-select"] }

# API & Networking
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# Security
keyring = "2.2"
age = "0.10"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# TUI
ratatui = "0.25"
crossterm = "0.27"
indicatif = "0.17"

# Utilities
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
regex = "1.10"
url = "2.5"
dirs = "5.0"
sled = "0.34"

[dev-dependencies]
mockito = "1.2"
tempfile = "3.8"
assert_cmd = "2.0"
predicates = "3.0"
pretty_assertions = "1.4"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

---

## 🏗️ Core Architecture Components

### Provider Trait (`src/providers/traits.rs`)

Define a clean, extensible Provider trait that all hosting platforms implement:

```rust
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    async fn test_connection(&self) -> Result<bool>;
    async fn create_repo(&self, config: RepoConfig) -> Result<Repository>;
    async fn get_repo(&self, name: &str) -> Result<Repository>;
    fn get_remote_url(&self, name: &str, protocol: Protocol) -> String;
    async fn create_branch(&self, repo: &str, branch: &str) -> Result<()>;
    async fn delete_branch(&self, repo: &str, branch: &str) -> Result<()>;
    async fn get_rate_limit(&self) -> Result<RateLimit>;
}

pub struct RepoConfig {
    pub name: String,
    pub description: Option<String>,
    pub private: bool,
    pub default_branch: String,
}

pub struct Repository {
    pub name: String,
    pub url: String,
    pub ssh_url: String,
    pub private: bool,
    pub default_branch: String,
}

pub enum Protocol { Https, Ssh }

pub struct RateLimit {
    pub limit: u32,
    pub remaining: u32,
    pub reset_at: chrono::DateTime<chrono::Utc>,
}
```

### Error Types (`src/utils/error.rs`)

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MultiGitError {
    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Authentication failed for {provider}: {reason}")]
    AuthError { provider: String, reason: String },
    
    #[error("Repository not found: {0}")]
    RepoNotFound(String),
    
    #[error("Remote '{0}' not configured")]
    RemoteNotFound(String),
    
    #[error("Conflict detected: {0}")]
    ConflictError(String),
    
    #[error("Rate limit exceeded for {provider}")]
    RateLimitError { provider: String },
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, MultiGitError>;
```

### Configuration (`src/core/config.rs`)

Implement hierarchical config loading (CLI > Repo > User > Default):

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub settings: Settings,
    
    #[serde(default)]
    pub sync: SyncConfig,
    
    #[serde(default)]
    pub security: SecurityConfig,
    
    #[serde(default)]
    pub remotes: HashMap<String, RemoteConfig>,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        // Load from ~/.config/multigit/config.toml
        // Then merge with .multigit/config.toml if exists
        let mut config = Self::load_user_config()?;
        if let Some(repo_config) = Self::load_repo_config()? {
            config = config.merge(repo_config);
        }
        Ok(config)
    }
    
    fn load_user_config() -> anyhow::Result<Config> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Cannot find config dir"))?;
        let path = config_dir.join("multigit/config.toml");
        
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Config::default())
        }
    }
    
    fn load_repo_config() -> anyhow::Result<Option<Config>> {
        let path = std::path::PathBuf::from(".multigit/config.toml");
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            Ok(Some(toml::from_str(&content)?))
        } else {
            Ok(None)
        }
    }
    
    fn merge(mut self, other: Config) -> Self {
        self.remotes.extend(other.remotes);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default = "default_branch")]
    pub default_branch: String,
    
    #[serde(default = "default_true")]
    pub parallel_push: bool,
    
    #[serde(default = "default_parallel")]
    pub max_parallel: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_branch: "main".into(),
            parallel_push: true,
            max_parallel: 4,
        }
    }
}

fn default_branch() -> String { "main".into() }
fn default_true() -> bool { true }
fn default_parallel() -> usize { 4 }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncConfig {
    #[serde(default)]
    pub auto_sync: bool,
    
    pub primary_source: Option<String>,
    
    #[serde(default = "default_strategy")]
    pub strategy: String,
}

fn default_strategy() -> String { "fast-forward".into() }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityConfig {
    #[serde(default = "default_auth")]
    pub auth_backend: String,
    
    #[serde(default)]
    pub audit_log: bool,
}

fn default_auth() -> String { "keyring".into() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
    pub username: String,
    pub api_url: Option<String>,
    
    #[serde(default = "default_true")]
    pub enabled: bool,
}
```

---

## 🎬 Implementation Phases & Task Breakdown

### Phase 1: Foundation (Week 1-2) - MVP Core

**Task 1.1: Project Setup**
- Create Cargo.toml with workspace structure
- Add .gitignore for Rust projects
- Create MIT LICENSE file
- Write initial README.md
- Set up GitHub Actions CI (test.yml)

**Task 1.2: Core Models** (`src/models/`)
- Define Repository, Remote, Config, SyncState structs
- Add serde derives and validation
- Write unit tests

**Task 1.3: Error Handling** (`src/utils/error.rs`)
- Define MultiGitError enum
- Implement Display and Error traits
- Add helpful error messages

**Task 1.4: Configuration System** (`src/core/config.rs`)
- Implement hierarchical config loading
- Support ~/.config/multigit/config.toml and .multigit/config.toml
- Add config merging and validation

**Task 1.5: Logging** (`src/utils/logger.rs`)
- Set up tracing/tracing-subscriber
- Support multiple log levels and formats

### Phase 2: Git Operations (Week 2-3)

**Task 2.1: Git Wrapper** (`src/git/operations.rs`)
- Wrap git2-rs for common operations
- Implement repo_open, repo_init, get_current_branch
- Add error conversion from git2::Error

**Task 2.2: Remote Management** (`src/git/remote.rs`)
- Implement add_remote, remove_remote, list_remotes
- Add remote URL generation (HTTPS/SSH)

**Task 2.3: Branch Operations** (`src/git/branch.rs`)
- Implement list_branches, create_branch, delete_branch
- Add branch comparison logic

### Phase 3: Provider System (Week 3-5)

**Task 3.1: Provider Trait** (`src/providers/traits.rs`)
- Define complete Provider trait
- Add documentation and common types

**Task 3.2: GitHub Provider** (`src/providers/github.rs`)
- Implement GitHub REST API v3 client
- Add PAT authentication
- Implement all Provider trait methods
- Add rate limiting

**Task 3.3: GitLab Provider** (`src/providers/gitlab.rs`)
- Implement GitLab API v4 client
- Handle GitLab-specific features

**Task 3.4: Bitbucket Provider** (`src/providers/bitbucket.rs`)
- Implement Bitbucket API 2.0 client
- Handle app password auth

**Task 3.5: Codeberg & Gitea** (`src/providers/codeberg.rs`, `gitea.rs`)
- Implement Gitea/Forgejo provider
- Support custom instance URLs

### Phase 4: Authentication & Security (Week 5-6)

**Task 4.1: Keyring Integration** (`src/security/keyring.rs`)
- Use keyring crate for OS-native credential storage
- Support macOS/Linux/Windows keyrings

**Task 4.2: Encrypted Fallback** (`src/security/encryption.rs`)
- Implement age-based encryption for credentials
- Handle passphrase prompting

**Task 4.3: Audit Logging** (`src/security/audit.rs`)
- Log all sensitive operations
- Implement log rotation

**Task 4.4: Auth Manager** (`src/core/auth.rs`)
- Create unified AuthManager interface
- Support keyring, encrypted file, env vars

### Phase 5: Sync Engine (Week 6-8)

**Task 5.1: Sync Manager** (`src/core/sync_manager.rs`)
- Implement push_all with parallel tokio tasks
- Add fetch_all with parallel fetching
- Track sync state using sled database

**Task 5.2: Conflict Detection**
- Implement detect_conflicts in sync_manager
- Check for divergent branches
- Generate conflict reports

**Task 5.3: Conflict Resolver** (`src/core/conflict_resolver.rs`)
- Implement multiple resolution strategies
- Add interactive conflict resolution

**Task 5.4: Health Checker** (`src/core/health_checker.rs`)
- Check repo status and remote connectivity
- Generate health reports with recommendations

### Phase 6: CLI Commands (Week 8-10)

**Task 6.1: CLI Parser** (`src/cli/parser.rs`)
- Set up clap with derive macros
- Define command structure

**Task 6.2-6.9: Implement Commands**
- `init.rs` - Initialize MultiGit
- `remote.rs` - Remote management (add/list/remove/test)
- `push.rs` - Push to remotes
- `pull.rs` - Pull from primary
- `sync.rs` - Full synchronization
- `status.rs` - Show sync status
- `conflict.rs` - Conflict management
- `doctor.rs` - Diagnostics and auto-fix

### Phase 7: UI & Progress (Week 10-11)

**Task 7.1: Progress Indicators** (`src/ui/progress.rs`)
- Use indicatif for progress bars
- Show per-remote progress with throughput

**Task 7.2: Output Formatting** (`src/ui/formatter.rs`)
- Implement table formatting
- Add colored output
- Support JSON output mode

**Task 7.3: Terminal UI** (`src/ui/tui.rs`)
- Implement interactive TUI with ratatui
- Create dashboard with real-time updates

### Phase 8: Daemon & Automation (Week 11-12)

**Task 8.1: Daemon Service** (`src/daemon/service.rs`)
- Implement background daemon process
- Handle signals for graceful shutdown

**Task 8.2: Scheduler** (`src/daemon/scheduler.rs`)
- Implement interval-based scheduling
- Support cron-like syntax

**Task 8.3: Daemon Commands** (`src/cli/commands/daemon.rs`)
- Implement daemon start/stop/status/logs

### Phase 9: Testing & Quality (Week 12-13)

**Task 9.1-9.4: Comprehensive Testing**
- Write unit tests (70% coverage target)
- Create integration tests with mocks
- Add E2E tests with fixtures
- Enhance CI/CD for multi-platform builds

### Phase 10: Documentation & Polish (Week 13-14)

**Task 10.1-10.3: Production Ready**
- Write comprehensive documentation
- Create installation scripts
- Set up release automation

---

## 🧪 Testing Examples

### Unit Test
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.settings.default_branch, "main");
    }
    
    #[tokio::test]
    async fn test_provider_connection() {
        let provider = MockProvider::new();
        assert!(provider.test_connection().await.is_ok());
    }
}
```

### Integration Test
```rust
#[tokio::test]
async fn test_push_to_multiple_remotes() {
    let temp_dir = TempDir::new().unwrap();
    let mut manager = SyncManager::new_for_testing();
    manager.add_provider("mock1", Box::new(MockProvider::new()));
    manager.add_provider("mock2", Box::new(MockProvider::new()));
    
    let results = manager.push_all("main").await.unwrap();
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| r.success));
}
```

---

## 📚 Key Implementation Examples

### GitHub Provider Snippet
```rust
pub struct GitHubProvider {
    client: Client,
    token: String,
    username: String,
}

#[async_trait]
impl Provider for GitHubProvider {
    fn name(&self) -> &str { "github" }
    
    async fn test_connection(&self) -> anyhow::Result<bool> {
        let response = self.client
            .get("https://api.github.com/user")
            .header("Authorization", format!("token {}", self.token))
            .header("User-Agent", "multigit")
            .send()
            .await?;
        Ok(response.status().is_success())
    }
    
    async fn create_repo(&self, config: RepoConfig) -> anyhow::Result<Repository> {
        let payload = serde_json::json!({
            "name": config.name,
            "description": config.description,
            "private": config.private,
        });
        
        let response = self.client
            .post("https://api.github.com/user/repos")
            .header("Authorization", format!("token {}", self.token))
            .header("User-Agent", "multigit")
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            anyhow::bail!("Failed to create repo: {}", response.text().await?);
        }
        
        // Parse response and return Repository
        let data: serde_json::Value = response.json().await?;
        Ok(Repository {
            name: data["name"].as_str().unwrap().to_string(),
            url: data["clone_url"].as_str().unwrap().to_string(),
            ssh_url: data["ssh_url"].as_str().unwrap().to_string(),
            private: data["private"].as_bool().unwrap(),
            default_branch: data["default_branch"].as_str().unwrap_or("main").to_string(),
        })
    }
}
```

### Parallel Push Implementation
```rust
impl SyncManager {
    pub async fn push_all(&self, branch: &str) -> anyhow::Result<Vec<PushResult>> {
        let mut tasks = Vec::new();
        
        for (name, provider) in &self.providers {
            let provider_name = name.clone();
            let provider = provider.clone();
            let branch = branch.to_string();
            
            tasks.push(tokio::spawn(async move {
                tracing::info!("Pushing to {}", provider_name);
                match provider.push(&branch).await {
                    Ok(_) => PushResult { 
                        provider: provider_name, 
                        success: true, 
                        message: "Success".into() 
                    },
                    Err(e) => PushResult { 
                        provider: provider_name, 
                        success: false, 
                        message: e.to_string() 
                    },
                }
            }));
        }
        
        let results = futures::future::join_all(tasks).await;
        Ok(results.into_iter().filter_map(Result::ok).collect())
    }
}
```

---

## 🎯 Success Criteria

Implementation is successful when:

1. ✅ **Core Commands Work**: init, remote, push, pull, sync, status
2. ✅ **Multi-Platform**: 3+ providers (GitHub, GitLab, Bitbucket)
3. ✅ **Secure**: OS keyring with encrypted fallback
4. ✅ **Fast**: Parallel operations outperform sequential
5. ✅ **Reliable**: Comprehensive error handling
6. ✅ **Tested**: 70%+ code coverage
7. ✅ **Documented**: Complete README and examples
8. ✅ **CI/CD**: Automated builds for Linux/macOS/Windows
9. ✅ **UX**: Progress bars, colors, helpful errors
10. ✅ **Quality**: Passes clippy, rustfmt, human review

---

## 🚀 Getting Started

1. Start with **Phase 1, Task 1.1** - Project Setup
2. Build incrementally - complete each task fully
3. Test continuously - write tests alongside code
4. Commit frequently - small, focused commits
5. Document as you go - add comments and rustdoc
6. Refer to `docs/project.md` and `docs/diagrams.md` for details
7. Follow `.windsurf/rules` for design principles

**Write code that looks human-written**: Include TODO comments, reasonable function lengths, thoughtful trade-offs, and natural variable names. Avoid perfect symmetry and over-engineering.

---

## 📖 References

- **Detailed Specs**: `docs/project.md`
- **Architecture**: `docs/diagrams.md`
- **Design Rules**: `.windsurf/rules`
- **Repository**: https://github.com/TIVerse/multigit (branch: master)

---

**Begin with Phase 1, Task 1.1. Good luck building MultiGit!** 🎉
