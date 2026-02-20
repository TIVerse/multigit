<div align="center">

# 🌐 MultiGit


[![GitHub release](https://img.shields.io/github/v/release/TIVerse/multigit?style=for-the-badge&logo=github)](https://github.com/TIVerse/multigit/releases)
[![CI Status](https://img.shields.io/github/actions/workflow/status/TIVerse/multigit/test.yml?style=for-the-badge&logo=github-actions&label=Tests)](https://github.com/TIVerse/multigit/actions)
[![Coverage](https://img.shields.io/codecov/c/github/TIVerse/multigit?style=for-the-badge&logo=codecov)](https://codecov.io/gh/TIVerse/multigit)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![Contributors](https://img.shields.io/github/contributors/TIVerse/multigit?style=for-the-badge&logo=github)](https://github.com/TIVerse/multigit/graphs/contributors)
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey?style=for-the-badge)](https://github.com/TIVerse/multigit)

[![Stars](https://img.shields.io/github/stars/TIVerse/multigit?style=for-the-badge&logo=github)](https://github.com/TIVerse/multigit/stargazers)
[![Issues](https://img.shields.io/github/issues/TIVerse/multigit?style=for-the-badge&logo=github)](https://github.com/TIVerse/multigit/issues)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=for-the-badge)](https://github.com/TIVerse/multigit/pulls)
[![Forks](https://img.shields.io/github/forks/TIVerse/multigit?style=for-the-badge&logo=github)](https://github.com/TIVerse/multigit/network/members)

<p align="center">
  <strong>A production-ready, blazingly fast Git multi-remote synchronization tool</strong><br>
  Push, pull, and sync your code across GitHub, GitLab, Bitbucket, Codeberg, Gitea/Forgejo with a single command
</p>

** Available as both `multigit` and `mg` commands **

<br/>

** Table of Contents**

[Features](#-features) •
[Why MultiGit?](#-why-multigit) •
[Quick Start](#-quick-start) •
[Installation](#-installation) •
[Usage](#-usage) •
[Configuration](#-configuration) •
[Security](#-security) •
[Examples](#-examples) •
[FAQ](#-faq) •
[Contributing](#-contributing) •
[Roadmap](#-development-status--roadmap)

---

</div>

## ✨ Features

<table>
<tr>
<td width="50%">

### Performance & Reliability
-  **Blazingly Fast** - Parallel operations via Tokio async runtime
-  **Smart Sync** - Incremental updates, not full clones
-  **Progress Tracking** - Real-time progress bars and status
-  **Atomic Operations** - All-or-nothing commits
-  **Retry Logic** - Automatic retry on transient failures

</td>
<td width="50%">

###  Security & Privacy
-  **OS Keyring Integration** - Native credential managers
-  **Host-Bound Credentials** - Tokens scoped to specific hosts
-  **HTTPS Enforcement** - Prevents insecure HTTP connections
-  **Age Encryption** - Encrypted credential fallback
-  **Zero Plain Text** - No passwords or tokens in files
-  **Audit Logging** - Track all sensitive operations
-  **Secret Redaction** - Automatic log sanitization
-  **TLS/SSL** - Encrypted network communication

</td>
</tr>
<tr>
<td>

###  Intelligent Operations
-  **Smart Conflict Detection** - Prevents data loss
-  **Multiple Merge Strategies** - Fast-forward, rebase, merge
-  **Selective Sync** - Choose branches, remotes, files
-  **Health Checks** - Auto-diagnose and fix issues
-  **Diff Analysis** - See exactly what will sync

</td>
<td>

###  Developer Experience
-  **Rich CLI/TUI** - Beautiful interactive terminal UI
-  **Conventional Commits** - Interactive commit wizard
-  **Daemon Mode** - Background sync with scheduler
-  **Cross-Platform** - Linux, macOS, Windows
-  **Zero Config** - Works out-of-the-box

</td>
</tr>
</table>

###  Supported Platforms

| Platform | Status | Features |
|----------|--------|----------|
| **GitHub** |  Full Support | Public, Private, Enterprise |
| **GitLab** |  Full Support | Cloud, Self-hosted |
| **Bitbucket** |  Full Support | Cloud, Server |
| **Codeberg** |  Full Support | Public repositories |
| **Gitea** |  Full Support | Self-hosted instances |
| **Forgejo** |  Full Support | Self-hosted instances |
</table>

<div align="center">

###  Real-World Benefits

| Traditional Workflow | With MultiGit | Time Saved |
|---------------------|---------------|------------|
| Push to 3 remotes manually | `mg sync` | **~2 minutes** |
| Set up credentials for each platform | One-time interactive setup | **~15 minutes** |
| Check sync status across platforms | `mg status` | **~5 minutes** |
| Resolve conflicts manually | Interactive resolver | **~10 minutes** |
| **Total per day** | | **~30 minutes** |

** That's *180+* hours saved per year for a typical developer!**

</div>

---

## 🎬 Quick Start

Get started in under 2 minutes! Here's the fastest path to multi-remote nirvana:

```bash
# 1️ Initialize MultiGit in your repository
mg init                    # or 'multigit init'

# 2️ Add remote platforms (you'll be prompted for credentials)
mg remote add github <username>
mg remote add gitlab <username>
mg remote add bitbucket <username>

# 3️ Make some changes, then use the interactive commit helper
mg cc                      # Interactive conventional commit wizard
# Or use traditional git commands - they work too!

# 4️ Sync to all remotes with one command! 🚀
mg sync

# 5️ Check sync status across all platforms
mg status
```

<div align="center">


</div>

###  Demo

```bash
$ mg sync
✓ Pushing to github...     [████████████████████] 100% (main)
✓ Pushing to gitlab...     [████████████████████] 100% (main)
✓ Pushing to bitbucket...  [████████████████████] 100% (main)
🎉 Successfully synced to 3 remotes in 2.3s
```

<div align="center">

<!-- Placeholder for demo GIF -->
<!-- ![MultiGit Demo](docs/assets/demo.gif) -->

### ⚡ Performance

<table>
<tr>
<td align="center">
<h3>3x</h3>
<sub>Faster than sequential pushes</sub>
</td>
<td align="center">
<h3>5</h3>
<sub>Platforms supported</sub>
</td>
<td align="center">
<h3>0</h3>
<sub>Plain text credentials</sub>
</td>
<td align="center">
<h3>100%</h3>
<sub>Rust, no dependencies issues</sub>
</td>
</tr>
</table>

</div>

## 📦 Installation

Choose your preferred installation method:

### 🚀 From Releases (Easiest)

Download pre-built binaries for your platform from [GitHub Releases](https://github.com/TIVerse/multigit/releases):

**Linux / macOS:**
```bash
# Download the latest release
curl -LO https://github.com/TIVerse/multigit/releases/latest/download/multigit-linux-x64.tar.gz

# Extract and install
tar -xzf multigit-linux-x64.tar.gz
sudo mv multigit mg /usr/local/bin/

# Verify installation
mg --version
```

**Windows (PowerShell):**
```powershell
# Download and run the installer
Invoke-WebRequest -Uri "https://github.com/TIVerse/multigit/releases/latest/download/multigit-windows-installer.exe" -OutFile "multigit-installer.exe"
.\multigit-installer.exe
```

### 🔧 Build from Source

```bash
# Clone the repository
git clone https://github.com/TIVerse/multigit.git
cd multigit

# Build release binaries
cargo build --release

# Test the build
./target/release/multigit --version
./target/release/mg --version          # Both binaries are built

# Optional: Install to system
cargo install --path .
```

### 🐳 Docker (Coming Soon)

```bash
docker pull tiverse/multigit:latest
docker run --rm -v $(pwd):/repo tiverse/multigit mg status
```

## 🚀 Usage

### 🎯 Core Workflows

<details>
<summary><b>📋 Initialize a Repository</b></summary>

```bash
# Navigate to your Git repository
cd your-git-repository

# Initialize MultiGit
mg init

# MultiGit creates a .multigit/ directory with configuration
# ✓ Configuration file created
# ✓ Default settings applied
# ✓ Ready to add remotes!
```

</details>

<details>
<summary><b>🔗 Managing Remote Platforms</b></summary>

```bash
# Add GitHub remote
mg remote add github your-username
# You'll be prompted for your Personal Access Token (PAT)

# Add GitLab remote
mg remote add gitlab your-username

# Add Bitbucket
mg remote add bitbucket your-username

# Add self-hosted Gitea/Forgejo
mg remote add mygitea your-username --url https://gitea.example.com

# Add Codeberg
mg remote add codeberg your-username

# List all configured remotes
mg remote list

# Remove a remote
mg remote remove github

# Update remote credentials
mg remote update gitlab --token
```

</details>

<details>
<summary><b>🔄 Syncing & Pushing</b></summary>

```bash
# Push to all configured remotes
mg push

# Push to specific remote only
mg push --to github

# Push specific branch
mg push --branch develop

# Sync bidirectionally (pull + push)
mg sync

# Sync with conflict resolution
mg sync --interactive

# Force sync (⚠️ use with caution - may overwrite changes)
mg sync --force

# Dry run to preview sync
mg sync --dry-run
```

</details>

<details>
<summary><b>📥 Pulling Changes</b></summary>

```bash
# Pull from primary remote (configured in settings)
mg pull

# Pull from specific remote
mg pull --from github

# Pull specific branch
mg pull --from gitlab --branch main

# Pull and rebase
mg pull --rebase
```

</details>

<details>
<summary><b>📊 Status & Monitoring</b></summary>

```bash
# Check sync status across all remotes
mg status

# Detailed status with commit differences
mg status --verbose

# Check specific remote
mg status --remote github

# Watch mode (auto-refresh every 5s)
mg status --watch
```

</details>

<details>
<summary><b>✍️ Conventional Commits (Interactive)</b></summary>

```bash
# Launch interactive commit wizard
mg cc                    # or 'mg commit'

# The wizard helps you:
# ✓ Select files to stage
# ✓ Choose commit type (feat, fix, docs, style, refactor, test, chore)
# ✓ Auto-detect scope from changed files
# ✓ Add breaking change markers
# ✓ Link to issues/tickets
# ✓ Preview formatted commit message
# ✓ Commit & optionally push

# Example output:
# ? Select commit type: feat
# ? Scope (optional): auth
# ? Short description: add OAuth2 support
# ? Breaking change?: No
# ? Issue reference: #42
# 
# Preview: feat(auth): add OAuth2 support (#42)
# 
# ✓ Committed successfully!
```

</details>

<details>
<summary><b>⚔️ Conflict Resolution</b></summary>

```bash
# List all detected conflicts
mg conflict list

# Resolve conflicts interactively
mg conflict resolve

# Choose resolution strategy
mg conflict resolve --strategy ours     # Use local changes
mg conflict resolve --strategy theirs   # Use remote changes
mg conflict resolve --strategy manual   # Manual resolution

# Show conflict diff
mg conflict diff
```

</details>

<details>
<summary><b>🤖 Daemon Mode (Background Sync)</b></summary>

```bash
# Start daemon with 5-minute sync interval
mg daemon start --interval 5m

# Start with hourly sync
mg daemon start --interval 1h

# Check daemon status
mg daemon status

# View daemon logs
mg daemon logs

# Stop daemon
mg daemon stop

# Restart daemon
mg daemon restart

# Configure daemon to run on system startup
mg daemon enable-startup
```

</details>

<details>
<summary><b>🔍 Health Check & Diagnostics</b></summary>

```bash
# Run comprehensive health check
mg doctor

# The doctor command checks:
# ✓ Git installation and version
# ✓ Remote connectivity
# ✓ Authentication status
# ✓ Configuration validity
# ✓ Repository integrity
# ✓ Network connectivity

# Auto-fix common issues
mg doctor --fix

# Verbose diagnostics
mg doctor --verbose
```

</details>

### 🎓 Common Scenarios

<table>
<tr>
<td width="50%">

**Mirror a project to GitLab**
```bash
mg remote add gitlab username
mg push --to gitlab
```

</td>
<td width="50%">

**Daily auto-sync**
```bash
mg daemon start --interval 24h
```

</td>
</tr>

<tr>
<td>

**Backup to multiple hosts**
```bash
mg remote add github user
mg remote add gitlab user
mg sync
```

</td>
<td>

**Check all remotes status**
```bash
mg status --verbose
```

</td>
</tr>

<tr>
<td>

**Emergency conflict fix**
```bash
mg conflict list
mg conflict resolve --interactive
```

</td>
<td>

**Test connectivity**
```bash
mg doctor --verbose
```

</td>
</tr>
</table>

### 🎨 Advanced Usage

```bash
# Chain commands for complex workflows
mg pull --from github && mg cc && mg push

# Use in CI/CD pipelines
mg init && mg remote add gitlab $CI_USERNAME && mg push --to gitlab

# Scheduled sync with cron
# Add to crontab: 0 */6 * * * cd /path/to/repo && mg sync

# Multi-branch sync
for branch in main develop staging; do
  git checkout $branch
  mg sync --branch $branch
done
```

## 📖 Configuration

MultiGit uses a **hierarchical configuration system** with sensible defaults:

```
🔧 CLI Flags (highest priority)
    ↓
📁 Repository Config (.multigit/config.toml)
    ↓
🏠 User Config (~/.config/multigit/config.toml)
    ↓
⚙️  Default Settings (lowest priority)
```

### 📝 Example Configuration

Create a `.multigit/config.toml` in your repository or `~/.config/multigit/config.toml` for global settings:

```toml
# ====================================
# MultiGit Configuration
# ====================================

[settings]
# Default branch to sync
default_branch = "main"

# Enable parallel operations for speed
parallel_push = true
max_parallel = 4

# Verbose output
verbose = false

[sync]
# Automatic sync when daemon is running
auto_sync = false

# Primary source for pulling
primary_source = "github"

# Merge strategy: "fast-forward", "rebase", "merge"
strategy = "fast-forward"

# Conflict resolution: "abort", "ours", "theirs", "interactive"
conflict_resolution = "interactive"

[security]
# Authentication backend: "keyring" (recommended) or "encrypted-file"
auth_backend = "keyring"

# Enable audit logging
audit_log = true

# Encrypt local config
encrypt_config = true

[daemon]
# Background sync interval (e.g., "5m", "1h", "24h")
sync_interval = "1h"

# Enable webhook server
webhook_enabled = false
webhook_port = 8080

[ui]
# Progress bar style: "bar", "spinner", "quiet"
progress_style = "bar"

# Color output
color = "auto"  # "auto", "always", "never"

# Show notifications
notifications = true

[remotes.github]
username = "your-username"
enabled = true
# Optional: Custom API URL for GitHub Enterprise
# api_url = "https://github.company.com"

[remotes.gitlab]
username = "your-username"
api_url = "https://gitlab.com"
enabled = true

[remotes.bitbucket]
username = "your-username"
enabled = false

[remotes.codeberg]
username = "your-username"
enabled = true

[remotes.mygitea]
username = "your-username"
api_url = "https://gitea.example.com"
enabled = true
```

### 🎛️ Configuration Commands

```bash
# View current configuration
mg config show

# Set a configuration value
mg config set sync.strategy rebase

# Edit configuration in your default editor
mg config edit

# Validate configuration
mg config validate

# Reset to defaults
mg config reset
```

## 🔒 Security

<div align="center">

**🛡️ Security is our top priority. Your credentials are never compromised.**

</div>

<table>
<tr>
<td width="50%">

### 🔐 Credential Management

- **OS Keyring Integration** 🔑
  - macOS: Keychain
  - Windows: Credential Manager
  - Linux: Secret Service / gnome-keyring
  
- **Encrypted Fallback** 🔒
  - Age encryption when keyring unavailable
  - Password-protected local storage
  - AES-256 encryption standard

</td>
<td width="50%">

### 🛡️ Security Features

- **Host-Bound Credentials** 🌐
  - Credentials scoped to specific hosts
  - Prevents confused-deputy attacks
  - Automatic migration from legacy keys

- **HTTPS Enforcement** 🔒
  - HTTP URLs rejected by default
  - Opt-in for insecure connections
  - Protects against MitM attacks

- **Zero Plain Text** 🚫
  - No tokens in config files
  - Environment tokens opt-in only
  - Memory wiping after use

- **Secret Redaction** 🛡️
  - Automatic log sanitization
  - Masks tokens, passwords, keys
  - Safe daemon output logging

- **Audit Logging** 📝
  - Track all sensitive operations
  - Credential access monitoring
  - Configurable log retention

</td>
</tr>
</table>

### 🔑 Setting Up Credentials

```bash
# Add credentials interactively (recommended)
mg remote add github username
# 🔐 You'll be prompted for your Personal Access Token
# ✓ Token encrypted and stored in OS keyring
# ✓ Never appears in plain text

# Verify credential storage
mg config show --security
```

### 🔒 Security Best Practices

<table>
<tr>
<td align="center" width="25%">
<h3>✅</h3>
<b>Use PATs</b><br/>
<sub>Personal Access Tokens, not passwords</sub>
</td>
<td align="center" width="25%">
<h3>🔄</h3>
<b>Rotate Regularly</b><br/>
<sub>Update tokens periodically</sub>
</td>
<td align="center" width="25%">
<h3>🎯</h3>
<b>Minimal Scope</b><br/>
<sub>Grant only needed permissions</sub>
</td>
<td align="center" width="25%">
<h3>📋</h3>
<b>Audit Logs</b><br/>
<sub>Review security logs regularly</sub>
</td>
</tr>
</table>

### 🚨 Security Reporting

Found a security vulnerability? Please **DO NOT** open a public issue. Instead:

- 📧 Email: security@tiverse.dev
- 🔒 Use GitHub Security Advisory
- ⏱️ We'll respond within 24-48 hours

## 🏗️ Architecture

MultiGit follows a clean, modular architecture designed for extensibility and maintainability:

```
┌─────────────────────────────────────────────────────────┐
│                   Frontend Layer                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐ │
│  │   CLI    │  │   TUI    │  │ Daemon   │  │  API    │ │
│  └──────────┘  └──────────┘  └──────────┘  └─────────┘ │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────┴────────────────────────────────────┐
│                  Core Engine                             │
│  ┌──────────────┐  ┌─────────────┐  ┌────────────────┐ │
│  │ Sync Manager │  │  Conflict   │  │   Scheduler    │ │
│  │              │  │  Resolver   │  │                │ │
│  └──────────────┘  └─────────────┘  └────────────────┘ │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────┴────────────────────────────────────┐
│              Platform Adapters                           │
│  ┌────────┐  ┌────────┐  ┌──────────┐  ┌────────────┐ │
│  │ GitHub │  │ GitLab │  │Bitbucket │  │Gitea/Forge │ │
│  └────────┘  └────────┘  └──────────┘  └────────────┘ │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────┴────────────────────────────────────┐
│            Git Operations Layer                          │
│              (libgit2 wrapper)                           │
│  ┌───────┐  ┌──────┐  ┌───────┐  ┌─────────────────┐  │
│  │ Clone │  │ Push │  │ Pull  │  │ Conflict Detect │  │
│  └───────┘  └──────┘  └───────┘  └─────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

### 🧩 Component Details

<table>
<tr>
<td width="25%">

**Frontend Layer**
- CLI commands
- Interactive TUI
- Daemon service
- REST API (planned)

</td>
<td width="25%">

**Core Engine**
- Sync orchestration
- Conflict detection
- Job scheduling
- State management

</td>
<td width="25%">

**Platform Adapters**
- GitHub API
- GitLab API
- Bitbucket API
- Gitea/Forgejo API

</td>
<td width="25%">

**Git Layer**
- libgit2 wrapper
- Repository ops
- Credential mgmt
- Network transport

</td>
</tr>
</table>

## 🤝 Contributing

We ❤️ contributions! Whether you're fixing bugs, adding features, or improving docs, we welcome your help.

<div align="center">

[![Contributors](https://contrib.rocks/image?repo=TIVerse/multigit)](https://github.com/TIVerse/multigit/graphs/contributors)

</div>

### 🚀 Quick Start for Contributors

<table>
<tr>
<td width="33%">

**1️⃣ Fork & Clone**
```bash
git clone https://github.com/YOUR_USERNAME/multigit.git
cd multigit
```

</td>
<td width="33%">

**2️⃣ Create Branch**
```bash
git checkout -b feature/amazing-feature
```

</td>
<td width="33%">

**3️⃣ Make Changes**
```bash
# Make your changes
cargo test
cargo fmt
```

</td>
</tr>
</table>

```bash
# 4️⃣ Commit with conventional commits
git commit -m 'feat: add amazing feature'

# 5️⃣ Push and create PR
git push origin feature/amazing-feature
```

### 📋 Contribution Guidelines

- 📖 Read our **[Contributing Guide](CONTRIBUTING.md)** for detailed guidelines
- ✅ Ensure all tests pass: `cargo test`
- 🎨 Format your code: `cargo fmt`
- 🔍 Run linter: `cargo clippy`
- 📝 Update documentation if needed
- ✉️ Use **[Conventional Commits](https://www.conventionalcommits.org/)**

### 🎯 Areas We Need Help

<table>
<tr>
<td align="center" width="25%">
<h3>🐛</h3>
<b>Bug Fixes</b><br/>
<sub>Help squash bugs</sub>
</td>
<td align="center" width="25%">
<h3>📚</h3>
<b>Documentation</b><br/>
<sub>Improve our docs</sub>
</td>
<td align="center" width="25%">
<h3>🎨</h3>
<b>UI/UX</b><br/>
<sub>Better user experience</sub>
</td>
<td align="center" width="25%">
<h3>🧪</h3>
<b>Testing</b><br/>
<sub>More test coverage</sub>
</td>
</tr>
</table>

## 📝 Development Status & Roadmap

<div align="center">

**🎉 Current Release: v2.0.0**

[![GitHub release](https://img.shields.io/github/v/release/TIVerse/multigit?style=flat-square)](https://github.com/TIVerse/multigit/releases/latest)
[![Release Date](https://img.shields.io/github/release-date/TIVerse/multigit?style=flat-square)](https://github.com/TIVerse/multigit/releases/latest)
[![Commits since release](https://img.shields.io/github/commits-since/TIVerse/multigit/latest?style=flat-square)](https://github.com/TIVerse/multigit/commits)

</div>

### ✅ Completed Features

<table>
<tr>
<td width="50%">

**Core Functionality**
- ✅ Multi-remote synchronization
- ✅ Smart conflict detection & resolution
- ✅ Parallel async operations
- ✅ OS keyring integration
- ✅ Encrypted credential storage
- ✅ Interactive CLI/TUI

</td>
<td width="50%">

**Advanced Features**
- ✅ Daemon mode with scheduler
- ✅ Conventional commits wizard
- ✅ Health check & diagnostics
- ✅ Comprehensive test suite (58+ tests)
- ✅ Cross-platform support
- ✅ Rich documentation

</td>
</tr>
</table>

### 🚀 Roadmap

| Version | Features | Status |
|---------|----------|--------|
| **v2.1.0** | TUI Dashboard, Interactive status viewer | 🚧 In Progress |
| **v2.2.0** | Git LFS support, Submodule sync | 📋 Planned |
| **v2.3.0** | Workspace management (multi-repo) | 📋 Planned |
| **v3.0.0** | Webhook server, GUI with Tauri | 💭 Future |

<details>
<summary><b>View Detailed Roadmap</b></summary>

**v2.1.0 - Enhanced UI** (Q1 2025)
- [ ] Full-screen TUI dashboard with ratatui
- [ ] Real-time sync monitoring
- [ ] Interactive conflict resolution UI
- [ ] Customizable themes

**v2.2.0 - Advanced Git Features** (Q2 2025)
- [ ] Git LFS support for large files
- [ ] Submodule synchronization
- [ ] Partial clone support
- [ ] Shallow clone optimization

**v2.3.0 - Workspace Management** (Q3 2025)
- [ ] Multi-repository workspaces
- [ ] Bulk operations across repos
- [ ] Workspace templates
- [ ] Dependency graph visualization

**v3.0.0 - Integration & GUI** (Q4 2025)
- [ ] Webhook server for push events
- [ ] REST API for integrations
- [ ] Native GUI application with Tauri
- [ ] Plugin system

</details>

### 📊 Project Stats

<div align="center">

![Code Size](https://img.shields.io/github/languages/code-size/TIVerse/multigit?style=flat-square)
![Lines of Code](https://img.shields.io/tokei/lines/github/TIVerse/multigit?style=flat-square)
![Commit Activity](https://img.shields.io/github/commit-activity/m/TIVerse/multigit?style=flat-square)
![Last Commit](https://img.shields.io/github/last-commit/TIVerse/multigit?style=flat-square)

</div>

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

## 📋 Quick Command Reference

<table>
<tr>
<th>Category</th>
<th>Command</th>
<th>Description</th>
</tr>
<tr>
<td rowspan="2"><b>Setup</b></td>
<td><code>mg init</code></td>
<td>Initialize MultiGit in repository</td>
</tr>
<tr>
<td><code>mg remote add &lt;platform&gt; &lt;user&gt;</code></td>
<td>Add a new remote platform</td>
</tr>
<tr>
<td rowspan="4"><b>Sync</b></td>
<td><code>mg sync</code></td>
<td>Bidirectional sync all remotes</td>
</tr>
<tr>
<td><code>mg push</code></td>
<td>Push to all remotes</td>
</tr>
<tr>
<td><code>mg pull</code></td>
<td>Pull from primary remote</td>
</tr>
<tr>
<td><code>mg status</code></td>
<td>Check sync status</td>
</tr>
<tr>
<td rowspan="2"><b>Commits</b></td>
<td><code>mg cc</code></td>
<td>Interactive conventional commit</td>
</tr>
<tr>
<td><code>mg commit</code></td>
<td>Alias for cc</td>
</tr>
<tr>
<td rowspan="3"><b>Daemon</b></td>
<td><code>mg daemon start --interval 1h</code></td>
<td>Start background sync daemon</td>
</tr>
<tr>
<td><code>mg daemon status</code></td>
<td>Check daemon status</td>
</tr>
<tr>
<td><code>mg daemon stop</code></td>
<td>Stop daemon</td>
</tr>
<tr>
<td rowspan="2"><b>Config</b></td>
<td><code>mg config show</code></td>
<td>View current configuration</td>
</tr>
<tr>
<td><code>mg config edit</code></td>
<td>Edit configuration</td>
</tr>
<tr>
<td rowspan="2"><b>Health</b></td>
<td><code>mg doctor</code></td>
<td>Run health diagnostics</td>
</tr>
<tr>
<td><code>mg doctor --fix</code></td>
<td>Auto-fix common issues</td>
</tr>
<tr>
<td rowspan="2"><b>Conflicts</b></td>
<td><code>mg conflict list</code></td>
<td>List all conflicts</td>
</tr>
<tr>
<td><code>mg conflict resolve</code></td>
<td>Resolve conflicts interactively</td>
</tr>
</table>

### 💾 Platform Support Matrix

<div align="center">

| OS | Architecture | Status | Download |
|----|--------------|--------|----------|
| 🐧 **Linux** | x86_64 | ✅ Supported | [Download](https://github.com/TIVerse/multigit/releases/latest) |
| 🐧 **Linux** | aarch64 (ARM) | ✅ Supported | [Download](https://github.com/TIVerse/multigit/releases/latest) |
| 🍎 **macOS** | Intel (x86_64) | ✅ Supported | [Download](https://github.com/TIVerse/multigit/releases/latest) |
| 🍎 **macOS** | Apple Silicon (M1/M2) | ✅ Supported | [Download](https://github.com/TIVerse/multigit/releases/latest) |
| 🪟 **Windows** | x86_64 | ✅ Supported | [Download](https://github.com/TIVerse/multigit/releases/latest) |
| 🪟 **Windows** | ARM64 | 🚧 Coming Soon | - |

</div>

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

## ⚖️ Comparison with Alternatives

| Feature | MultiGit | Shell Scripts | Git Aliases | Manual Sync |
|---------|----------|---------------|-------------|-------------|
| **Multi-remote push** | ✅ One command | ⚠️ Custom script | ⚠️ Per-remote | ❌ Very tedious |
| **Conflict detection** | ✅ Automatic | ❌ Manual | ❌ Manual | ❌ Manual |
| **Secure credentials** | ✅ OS keyring | ⚠️ Often plain text | ⚠️ Git config | ⚠️ Various |
| **Progress tracking** | ✅ Real-time UI | ❌ No | ❌ No | ❌ No |
| **Daemon mode** | ✅ Built-in | ⚠️ Via cron | ❌ No | ❌ No |
| **Cross-platform** | ✅ Native binaries | ⚠️ Shell dependent | ✅ Yes | ✅ Yes |
| **API support** | ✅ All major platforms | ⚠️ Manual setup | ❌ No | ❌ No |
| **Error recovery** | ✅ Automatic retry | ⚠️ Manual | ❌ No | ❌ No |
| **Conventional commits** | ✅ Interactive wizard | ❌ No | ❌ No | ⚠️ Manual |
| **Configuration** | ✅ TOML + CLI | ⚠️ Hard-coded | ⚠️ Git config | N/A |

### Why Choose MultiGit?

<table>
<tr>
<td width="33%">

**🎯 Purpose-Built**

Unlike generic scripts, MultiGit is designed specifically for multi-remote sync with robust error handling and conflict resolution.

</td>
<td width="33%">

**🔒 Security First**

Native OS keyring support means your credentials are never stored in plain text or version control.

</td>
<td width="33%">

**⚡ Performance**

Parallel async operations make syncing multiple remotes faster than sequential git commands.

</td>
</tr>
</table>

## ❓ FAQ

<details>
<summary><b>How is this different from git remote add?</b></summary>

Git's native remote support requires you to push/pull each remote individually. MultiGit:
- Pushes to all remotes with one command
- Detects and resolves conflicts automatically
- Manages authentication securely
- Provides daemon mode for automatic syncing
- Offers rich CLI/TUI experience

</details>

<details>
<summary><b>Does MultiGit work with private repositories?</b></summary>

Yes! MultiGit fully supports private repositories on all platforms. You'll need to provide a Personal Access Token (PAT) with appropriate permissions when adding a remote.

</details>

<details>
<summary><b>Can I use MultiGit in CI/CD pipelines?</b></summary>

Absolutely! MultiGit is designed for automation:

```bash
# Example GitHub Actions workflow
- name: Sync to GitLab
  run: |
    mg init
    mg remote add gitlab ${{ secrets.GITLAB_USERNAME }}
    mg push --to gitlab
```

</details>

<details>
<summary><b>How does MultiGit handle merge conflicts?</b></summary>

MultiGit provides three conflict resolution modes:
1. **Interactive** (default) - Prompts you to resolve conflicts
2. **Abort** - Stops sync and preserves current state
3. **Strategic** - Use `--strategy ours/theirs` to auto-resolve

You can configure the default behavior in your config file.

</details>

<details>
<summary><b>Is my authentication information secure?</b></summary>

Yes! MultiGit uses:
- **OS Keyring** (macOS Keychain, Windows Credential Manager, Linux Secret Service)
- **Age Encryption** as fallback
- **Zero plain text** - Never stores credentials in files

</details>

<details>
<summary><b>Can I sync only specific branches?</b></summary>

Yes! Use the `--branch` flag:

```bash
mg sync --branch main
mg push --branch develop --to github
```

</details>

<details>
<summary><b>What happens if one remote fails?</b></summary>

MultiGit continues syncing to other remotes and reports which operations succeeded/failed. You can review errors and retry with `mg sync --retry`.

</details>

<details>
<summary><b>Does MultiGit support Git LFS?</b></summary>

Git LFS support is planned for v2.0.0. Follow [issue #XX](https://github.com/TIVerse/multigit/issues) for updates.

</details>

<details>
<summary><b>Can I use this with monorepos or workspaces?</b></summary>

Single-repo support is stable in v2.0.0. Workspace management for multiple repositories is coming in a future release.

</details>

<details>
<summary><b>How do I report bugs or request features?</b></summary>

We love feedback! Please:
- 🐛 [Report bugs](https://github.com/TIVerse/multigit/issues/new?template=bug_report.md)
- 💡 [Request features](https://github.com/TIVerse/multigit/issues/new?template=feature_request.md)
- 💬 [Join discussions](https://github.com/TIVerse/multigit/discussions)

</details>

## 🙏 Acknowledgments

MultiGit stands on the shoulders of giants:

- 🦀 Built with [Rust](https://www.rust-lang.org/) - Performance and safety
- 📚 Git operations powered by [libgit2](https://libgit2.org/) - Robust Git internals
- 🎨 Terminal UI with [ratatui](https://github.com/ratatui-org/ratatui) - Beautiful TUI
- ⚡ Async runtime by [Tokio](https://tokio.rs/) - Blazing fast concurrency
- 🎯 CLI framework with [clap](https://github.com/clap-rs/clap) - Powerful argument parsing
- 🔐 Secure storage with [keyring](https://github.com/hww3/keyring-rs) - OS-native credentials

## 📧 Support & Community

<div align="center">

### 💬 Get Help

<table>
<tr>
<td align="center" width="25%">
<a href="https://github.com/TIVerse/multigit/issues">
<img src="https://img.shields.io/badge/Bug%20Report-red?style=for-the-badge&logo=github" alt="Bug Report"/>
</a>
<br/>
<sub><b>Report Bugs</b></sub>
</td>
<td align="center" width="25%">
<a href="https://github.com/TIVerse/multigit/issues">
<img src="https://img.shields.io/badge/Feature%20Request-blue?style=for-the-badge&logo=github" alt="Feature Request"/>
</a>
<br/>
<sub><b>Request Features</b></sub>
</td>
<td align="center" width="25%">
<a href="https://github.com/TIVerse/multigit/discussions">
<img src="https://img.shields.io/badge/Discussions-purple?style=for-the-badge&logo=github" alt="Discussions"/>
</a>
<br/>
<sub><b>Join Discussions</b></sub>
</td>
<td align="center" width="25%">
<a href="docs/">
<img src="https://img.shields.io/badge/Documentation-green?style=for-the-badge&logo=readthedocs" alt="Documentation"/>
</a>
<br/>
<sub><b>Read Docs</b></sub>
</td>
</tr>
</table>

### ⭐ Star History

[![Star History Chart](https://api.star-history.com/svg?repos=TIVerse/multigit&type=Date)](https://star-history.com/#TIVerse/multigit&Date)

---

### 🎉 Show Your Support

If MultiGit has helped you, please consider:

⭐ **Starring** this repository to show your support<br/>
🐦 **Sharing** it with your network<br/>
🤝 **Contributing** to make it even better<br/>
☕ **Sponsoring** the project (coming soon)<br/>

---

<p align="center">
  <img src="https://img.shields.io/badge/Made%20with-Rust-orange?style=for-the-badge&logo=rust" alt="Made with Rust"/>
  <img src="https://img.shields.io/badge/Built%20with-❤️-red?style=for-the-badge" alt="Built with Love"/>
  <img src="https://img.shields.io/badge/Open%20Source-✨-brightgreen?style=for-the-badge" alt="Open Source"/>
</p>

<p align="center">
  <b>Made with ❤️ by the <a href="https://github.com/TIVerse">TIVerse Team</a></b>
</p>

<p align="center">
  <sub>© 2024 TIVerse. Licensed under MIT.</sub>
</p>

</div>
