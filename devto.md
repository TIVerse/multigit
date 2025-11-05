---
title: "MultiGit: One repository. Infinite destinations."
published: false
description: "A production-ready, cross-platform multi-remote Git tool built in Rust. Push, pull, and sync across GitHub, GitLab, Bitbucket, Codeberg, and Gitea with a single command."
tags: rust, git, devops, opensource, productivity
---

MultiGit is a production-ready, cross-platform Git multi-remote synchronization tool built in Rust. It lets you push, pull, and sync your repository across multiple Git hosting platforms—GitHub, GitLab, Bitbucket, Codeberg, and Gitea/Forgejo—with one command.

Available as both `multigit` and `mg` commands for your convenience.

Repository: https://github.com/TIVerse/multigit
Crate: https://crates.io/crates/multigit
Docs: https://docs.rs/multigit

# Why MultiGit?

- You want redundancy and resilience across platforms.
- You contribute to both public (GitHub) and internal (GitLab) repos.
- You need reliable backups and continuous mirroring.
- You want a single workflow to manage many platforms without brittle scripts.

# Highlights

- Multi-Remote Sync: push/pull/sync to/from multiple remotes simultaneously
- Secure by Default: OS keyring integration with encrypted fallback
- Fast and Concurrent: Tokio-based async operations with proper concurrency limits
- Smart Conflict Handling: detection, reporting, and guided resolution strategies
- Daemon Mode: background sync with scheduling, logs, and health checks
- Rich CLI UX: progress indicators, pretty output, and JSON mode for automation
- Cross-Platform: Linux, macOS, and Windows

Backed by a comprehensive test suite (unit + integration + workflow) and CI.

# Quick Start

Install from crates.io:

```bash
cargo install multigit
```

Build from source:

```bash
git clone https://github.com/TIVerse/multigit.git
cd multigit
cargo build --release
./target/release/multigit --version
./target/release/mg --version
```

Initialize in a repository and add remotes:

```bash
cd your-repo
mg init

mg remote add github your-username
mg remote add gitlab your-username

# Commit with the interactive Conventional Commit helper
mg cc

# Sync to all remotes
mg sync

# Check status across remotes
mg status
```

Tip: Use `mg` for quick typing or `multigit` in scripts—they are identical binaries.

# Core Features

- Multi-remote operations: `push`, `pull`, `fetch`, `sync`
- Remote lifecycle: `remote add/remove/list/test/update`
- Conflicts: `conflict list` and interactive `conflict resolve`
- Automation: `daemon start/stop/status/logs` with interval-based scheduling
- Health and diagnostics: `doctor` to auto-detect common issues
- Developer UX: `cc` (Conventional Commit helper), `log`, `switch`, `stash`, `undo`, `amend`, `changelog`
- Compatibility: pass-through to native git for everything else

# Usage Cheatsheet

Initialize and add platforms:

```bash
multigit init
multigit remote add github your-username
multigit remote add gitlab your-username
multigit remote add bitbucket your-username
multigit remote add mygitea your-username --url https://gitea.example.com
```

Sync safely:

```bash
# Interactive sync with conflict detection
multigit sync

# Dry run to preview
multigit sync --dry-run

# To force push history (use with caution), use push or mirror:
multigit push --force
multigit mirror --force
```

Conflicts:

```bash
multigit conflict list
multigit conflict resolve
```

Daemon mode:

```bash
multigit daemon start --interval 5
multigit daemon status
multigit daemon logs --lines 100
multigit daemon stop
```

Conventional commits (interactive):

```bash
mg cc
```

JSON output for automation:

```bash
multigit --json status
multigit --json remote list
```

# Configuration

MultiGit uses a hierarchical configuration system:

1. CLI Flags (highest priority)
2. Repository config: `.multigit/config.toml` (project)
3. User config: `~/.config/multigit/config.toml` (global)
4. Built-in defaults (lowest)

Example `~/.config/multigit/config.toml`:

```toml
[settings]
default_branch = "main"
parallel_push = true
max_parallel = 4
colored_output = true

[sync]
auto_sync = false
strategy = "fast-forward"
detect_conflicts = true

[security]
auth_backend = "keyring"  # or "encrypted-file"
audit_log = true

[remotes.github]
username = "your-username"
enabled = true
provider = "github"

[remotes.gitlab]
username = "your-username"
api_url = "https://gitlab.com"
enabled = true
provider = "gitlab"
```

Your repository-local config (created by `mg init`) lives at `.multigit/config.toml`.

# Security Model

- OS Keyring Integration: macOS Keychain, Windows Credential Manager, Linux Secret Service
- Encrypted Fallback: age-encrypted local storage when keyring is unavailable
- No Plain-Text Secrets: tokens are never stored in plain text
- Audit Logging: optional, configurable

Credentials are stored when you add a remote:

```bash
multigit remote add github your-username
# You’ll be prompted for a token, stored securely
```

# Architecture (High-Level)

MultiGit follows a modular architecture with a clear separation of concerns. Source files referenced are in the `src/` tree.

```mermaid
graph TB
  UI[CLI Commands] --> CORE[Core Engine]
  CORE --> PROVIDERS[Provider Layer]
  CORE --> GIT[Git Operations (git2)]
  CORE --> SECURITY[Security]
  CORE --> DAEMON[Daemon & Scheduler]

  subgraph Core
    CONFIG[core/config.rs]
    AUTH[core/auth.rs]
    SYNC[core/sync_manager.rs]
    CONFLICT[core/conflict_resolver.rs]
    HEALTH[core/health_checker.rs]
  end

  subgraph Providers
    FACTORY[providers/factory.rs]
    GH[providers/github.rs]
    GL[providers/gitlab.rs]
    BB[providers/bitbucket.rs]
    CB[providers/codeberg.rs]
    GE[providers/gitea.rs]
  end

  subgraph Daemon
    SERVICE[daemon/service.rs]
    SCHED[daemon/scheduler.rs]
  end

  UI --> CONFIG
  CONFIG --> SYNC
  AUTH --> PROVIDERS
  SYNC --> GIT
  SYNC --> PROVIDERS
  SYNC --> CONFLICT
  SERVICE --> SYNC
```

Notable modules:

- `src/main.rs`: CLI command tree (push/pull/fetch/sync/status/remote/daemon/conflict/etc.)
- `src/core/`: configuration, authentication, sync orchestration, conflict resolution
- `src/providers/`: provider trait + concrete implementations (GitHub, GitLab, Bitbucket, Codeberg, Gitea), created via `providers::factory`
- `src/daemon/`: daemon service and scheduler
- `src/git/`: thin wrapper around `git2` for reliable Git operations
- `src/ui/`: formatting, progress indicators, output helpers
- `src/utils/`: error handling, logging (`tracing`), helpers

# How Sync Works

The `sync` command orchestrates a safe flow:

1. Load config and current branch
2. Ensure working directory is clean
3. Fetch from all enabled remotes
4. Detect conflicts/divergence
5. Push to all remotes in parallel with sane concurrency limits
6. Report per-remote results and update state

```rust
// src/cli/commands/sync.rs (excerpt)
let manager = SyncManager::new(".")?.with_max_parallel(config.settings.max_parallel);
let enabled: Vec<String> = config.enabled_remotes().keys().cloned().collect();
let fetch_results = manager.fetch_all(&enabled).await?;
let push_results  = manager.push_all(&branch_name, &enabled).await?;
```

# Interactive Setup & Conventional Commits

- `multigit setup` launches a friendly wizard: select providers, guided token prompts, connection testing, and config persistence. See `src/cli/commands/setup.rs`.
- `mg cc` opens an interactive Conventional Commit helper with:
  - File staging (All or Selectively)
  - Type, scope (auto-suggestions), and description validation
  - Optional body, breaking change flag, and footer (issues/refs)
  - Preview and edit-before-commit

# Daemon Mode

Run background syncs on a schedule:

```bash
multigit daemon start --interval 5
multigit daemon status
multigit daemon logs --lines 100
```

Under the hood (`src/daemon/`):

- Interval-based scheduler
- Proper termination and PID handling
- Invokes real syncs and logs results

For a minimal reproducible pattern, see `examples/scheduler_example.rs`.

# Examples

- `examples/basic_usage.rs`: configure remotes with the `Config` API
- `examples/scheduler_example.rs`: periodic tasks with the scheduler
- `examples/ui_formatting.rs`: pretty terminal output helpers

Run examples:

```bash
cargo run --example basic_usage
cargo run --example scheduler_example
cargo run --example ui_formatting
```

# Testing & CI

- Tests: `tests/unit/` and `tests/integration/` cover core logic, providers, UI formatting, sync manager, and workflows
- CI: GitHub Actions build, test, and verify on each change
- Developer utilities: `verify.sh`, coverage reports, and a structured changelog

# Performance & Reliability

- Concurrency: tokio + semaphore-based parallelism for bounded concurrency
- Timeouts: network operations include sensible timeouts and progress callbacks
- Accurate metrics: fetch/push feedback and improved commit counting
- Robust error model: typed errors (`anyhow`, `thiserror`) and structured logging (`tracing`)

# Roadmap

- Terminal UI (TUI) dashboard with `ratatui`
- Workspace management for multiple repos
- Git LFS and Submodules
- Webhook server for push notifications
- GUI app (Tauri)

Contributions are welcome—check `CONTRIBUTING.md` and open a PR.

# Get Started Today

- Install: `cargo install multigit`
- Initialize: `mg init`
- Add providers: `mg remote add <provider> <username>`
- Sync: `mg sync`

If you find MultiGit useful, star the repo and share feedback or ideas. Happy syncing!
