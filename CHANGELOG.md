# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-01-30

### Added

#### Core Features
- Multi-remote Git synchronization across 5 platforms (GitHub, GitLab, Bitbucket, Codeberg, Gitea/Forgejo)
- Hierarchical configuration system (repository, user, CLI flags)
- Secure credential management with OS keyring integration
- Parallel async operations powered by Tokio
- Smart conflict detection and resolution strategies

#### CLI Commands
- `multigit init` - Initialize MultiGit in a repository
- `multigit remote add/remove/list/test/update` - Manage Git hosting remotes
- `multigit push/pull/fetch/sync` - Git operations across multiple remotes
- `multigit status` - Show sync status across all remotes
- `multigit conflict detect/resolve` - Handle divergent branches
- `multigit branch/tag` - Branch and tag management
- `multigit daemon start/stop/status/logs` - Background daemon for automation
- `multigit doctor` - Diagnose and fix issues

#### User Interface
- Multi-progress bars for parallel operations using indicatif
- Beautiful table formatting with auto-sizing columns
- Colored terminal output with ANSI escape codes
- JSON output mode for scripting (`--json` flag)
- Interactive prompts with dialoguer
- Rich status indicators (✓ ✗ ⚠ ℹ ●)

#### Daemon & Automation
- Background daemon service with PID file management
- Interval-based scheduling (5m, 1h, 30s format)
- Graceful shutdown with signal handling (Unix)
- Automatic sync at configurable intervals
- Health monitoring and error resilience

#### Security
- OS-native keyring integration (macOS Keychain, Windows Credential Manager, Linux Secret Service)
- Age-encrypted credential fallback storage
- Audit logging for sensitive operations
- No plain-text credential storage
- Environment variable support for CI/CD

#### Provider Support
- **GitHub**: REST API v3 with PAT authentication
- **GitLab**: API v4 with custom instance URLs
- **Bitbucket**: API 2.0 with app password auth
- **Codeberg**: Gitea/Forgejo API support
- **Gitea**: Self-hosted instance support with custom URLs

#### Testing & Quality
- 58 comprehensive tests (unit, integration, workflow)
- Test fixtures and mock data generators
- Continuous integration with GitHub Actions
- Cross-platform support (Linux, macOS, Windows)
- Example code for common workflows

### Developer Experience
- Extensive rustdoc API documentation
- User guides and tutorials
- Example programs demonstrating key features
- CONTRIBUTING guide for contributors
- Modular architecture for easy extension

### Technical Details
- Built with Rust 🦀 for safety and performance
- Async/await with Tokio for concurrency
- libgit2 bindings for Git operations
- reqwest for HTTP API calls
- Structured logging with tracing
- TOML configuration format

## [Unreleased]

### Planned
- Terminal UI (TUI) with ratatui dashboard
- Workspace management for multiple repositories
- Git LFS support
- Submodule synchronization
- Webhook server for push notifications
- GUI application with Tauri

---

## Version History

### Pre-1.0.0 Development (Sprints)

**Sprint 1** (Foundation)
- Core configuration system
- Authentication management
- Remote and conflict CLI commands
- Interactive prompts

**Sprint 2** (User Interface)
- Progress indicators
- Output formatting
- Table generation
- Color support

**Sprint 3** (Automation)
- Daemon service
- Task scheduler
- Background sync

**Sprint 4** (Quality)
- Comprehensive test suite
- Unit and integration tests
- Test fixtures

**Sprint 5** (Documentation)
- API documentation
- User guides
- Examples
- Release preparation

---

[1.0.0]: https://github.com/TIVerse/multigit/releases/tag/v1.0.0
