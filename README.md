# MultiGit

> **One repository. Infinite destinations.**

MultiGit is a production-ready, cross-platform Git multi-remote synchronization tool built in Rust. Push, pull, and sync your code across multiple Git hosting platforms (GitHub, GitLab, Bitbucket, Codeberg, Gitea/Forgejo) with a single command.

**Available as both `multigit` and `mg` commands for your convenience!**

[![CI Status](https://github.com/TIVerse/multigit/workflows/test/badge.svg)](https://github.com/TIVerse/multigit/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/multigit.svg)](https://crates.io/crates/multigit)
[![Documentation](https://docs.rs/multigit/badge.svg)](https://docs.rs/multigit)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

## ✨ Features

- 🚀 **Multi-Remote Sync** - Push/pull to multiple Git hosts simultaneously
- 🔒 **Secure by Default** - OS keyring integration, encrypted credential storage
- ⚡ **Blazingly Fast** - Parallel operations powered by Tokio async runtime
- 🎯 **Smart Conflict Detection** - Intelligent merge strategies to prevent data loss
- 📊 **Rich CLI/TUI** - Beautiful progress bars and interactive terminal UI
- 🤖 **Daemon Mode** - Background sync with scheduling and webhooks
- 🌍 **Cross-Platform** - Linux, macOS, and Windows support
- 🔧 **Zero Config** - Sensible defaults, configuration when you need it

## 🎬 Quick Start

```bash
# Initialize MultiGit in your repository
mg init                    # or 'multigit init'

# Add remote platforms
mg remote add github <username>
mg remote add gitlab <username>

# Make some changes, then use the interactive commit helper
mg cc                      # Interactive conventional commit wizard

# Sync to all remotes
mg sync

# Check status
mg status
```

**💡 Tip**: Use `mg` for quick commands or `multigit` for scripts - they're identical!

## 📦 Installation

### From Source (Cargo)

```bash
cargo install multigit
```

**Note**: This installs both `multigit` and `mg` binaries.

### From GitHub Releases

Download pre-built binaries from [Releases](https://github.com/TIVerse/multigit/releases).

### Build from Source

```bash
git clone https://github.com/TIVerse/multigit.git
cd multigit
cargo build --release
./target/release/multigit --version
./target/release/mg --version          # Both binaries are built
```

## 🚀 Usage

### Initialize MultiGit

```bash
cd your-git-repository
multigit init
```

### Add Git Hosting Platforms

```bash
# GitHub
multigit remote add github your-username

# GitLab
multigit remote add gitlab your-username

# Bitbucket
multigit remote add bitbucket your-username

# Self-hosted Gitea
multigit remote add mygitea your-username --url https://gitea.example.com
```

### Push to All Remotes

```bash
multigit push
```

### Pull from Primary Remote

```bash
multigit pull --from github
```

### Sync All Remotes

```bash
# Interactive sync with conflict resolution
multigit sync

# Force sync (use with caution)
multigit sync --force
```

### Check Sync Status

```bash
multigit status
```

### Conventional Commits

```bash
# Interactive conventional commit helper
mg cc                    # or 'multigit cc' or 'mg commit'

# Features:
# - Select files to stage
# - Choose commit type (feat, fix, docs, etc.)
# - Smart scope detection from changed files
# - Breaking change detection
# - Issue reference linking
# - Preview before committing
```

### Manage Conflicts

```bash
# List detected conflicts
multigit conflict list

# Resolve conflicts interactively
multigit conflict resolve
```

### Daemon Mode

```bash
# Start background daemon
multigit daemon start --interval 5m

# Check daemon status
multigit daemon status

# Stop daemon
multigit daemon stop
```

### Health Check

```bash
# Diagnose issues and auto-fix
multigit doctor
```

## 📖 Configuration

MultiGit uses a hierarchical configuration system:

1. **Repository Config**: `.multigit/config.toml` (project-specific)
2. **User Config**: `~/.config/multigit/config.toml` (global defaults)
3. **CLI Flags**: Override any configuration

### Example Configuration

```toml
[settings]
default_branch = "main"
parallel_push = true
max_parallel = 4

[sync]
auto_sync = false
primary_source = "github"
strategy = "fast-forward"

[security]
auth_backend = "keyring"  # or "encrypted-file"
audit_log = true

[remotes.github]
username = "your-username"
enabled = true

[remotes.gitlab]
username = "your-username"
api_url = "https://gitlab.com"
enabled = true
```

## 🔒 Security

MultiGit prioritizes security:

- **OS Keyring Integration**: Uses native credential managers (macOS Keychain, Windows Credential Manager, Linux Secret Service)
- **Encrypted Fallback**: Age-encrypted credential storage when keyring unavailable
- **No Plain Text Secrets**: Never stores tokens or passwords in plain text
- **Audit Logging**: Tracks all sensitive operations

### Setting Up Credentials

```bash
# Add credentials interactively (recommended)
multigit remote add github username
# You'll be prompted for your personal access token

# Credentials are stored securely in your OS keyring
```

## 🏗️ Architecture

MultiGit follows a modular architecture:

```
Core Engine (sync_manager, conflict_resolver)
    ↓
Platform Adapters (github, gitlab, bitbucket, gitea)
    ↓
Git Operations Layer (libgit2 wrapper)
    ↓
Frontends (CLI, TUI, future: GUI)
```

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 Development Status

**🎉 MultiGit v1.0.0 Released!**

MultiGit has reached v1.0.0 with all core features implemented. See our [CHANGELOG](CHANGELOG.md) for details.

**Completed:**
- ✅ Sprint 1: Remote & conflict commands + interactive prompts
- ✅ Sprint 2: UI layer with progress bars & formatting  
- ✅ Sprint 3: Daemon service, scheduler & CLI commands
- ✅ Sprint 4: Comprehensive test suite (58 passing tests)
- ✅ Sprint 5: Documentation, examples & release prep

**Roadmap (v2.0.0+):**
- [ ] Terminal UI (TUI) dashboard with ratatui
- [ ] Workspace management for multiple repos
- [ ] Git LFS support
- [ ] Submodule synchronization
- [ ] Webhook server for push notifications
- [ ] GUI application with Tauri

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📚 Examples

Check out the [examples/](examples/) directory for complete working examples:

- **[basic_usage.rs](examples/basic_usage.rs)** - Configuration and remote management
- **[scheduler_example.rs](examples/scheduler_example.rs)** - Periodic task scheduling
- **[ui_formatting.rs](examples/ui_formatting.rs)** - Beautiful terminal output

Run examples with:
```bash
cargo run --example basic_usage
cargo run --example scheduler_example
cargo run --example ui_formatting
```

## 🎯 Use Cases

**Open Source Maintainers**
- Mirror repositories across GitHub, GitLab, and self-hosted platforms
- Ensure availability if one platform goes down
- Reach wider audiences on different platforms

**Enterprise Teams**
- Maintain internal GitLab and external GitHub repos in sync
- Backup to multiple locations automatically
- Comply with data residency requirements

**Individual Developers**
- Keep personal and work accounts synchronized
- Showcase work on multiple platforms for visibility
- Automatic backups with daemon mode

**CI/CD Pipelines**
- Deploy to multiple hosting platforms from one source
- Test across different Git hosting APIs
- Automated synchronization in workflows

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Git operations powered by [libgit2](https://libgit2.org/)
- Terminal UI with [ratatui](https://github.com/ratatui-org/ratatui)
- Async runtime by [Tokio](https://tokio.rs/)
- CLI framework with [clap](https://github.com/clap-rs/clap)

## 📧 Support

- 🐛 [Report a Bug](https://github.com/TIVerse/multigit/issues)
- 💡 [Request a Feature](https://github.com/TIVerse/multigit/issues)
- 📖 [Documentation](docs/)
- 💬 [Discussions](https://github.com/TIVerse/multigit/discussions)

---

**Made with ❤️ by the TIVerse Team**
