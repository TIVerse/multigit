# MultiGit User Guide

Complete guide to using MultiGit for multi-remote Git synchronization.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [Remote Management](#remote-management)
- [Synchronization](#synchronization)
- [Conflict Resolution](#conflict-resolution)
- [Daemon Mode](#daemon-mode)
- [Advanced Usage](#advanced-usage)
- [Troubleshooting](#troubleshooting)

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/TIVerse/multigit.git
cd multigit

# Build and install
cargo install --path .

# Verify installation
multigit --version
```

### From Cargo

```bash
cargo install multigit
```

## Quick Start

### 1. Initialize MultiGit

```bash
cd your-repository
multigit init
```

This creates a `.multigit/config.toml` file in your repository.

### 2. Add Remote Platforms

```bash
# Add GitHub
multigit remote add github your-username

# Add GitLab
multigit remote add gitlab your-username

# Add self-hosted Gitea
multigit remote add mygitea your-username --url https://git.example.com
```

You'll be prompted to enter your personal access tokens securely.

### 3. Push to All Remotes

```bash
multigit push
```

### 4. Check Sync Status

```bash
multigit status
```

## Configuration

MultiGit uses a hierarchical configuration system:

1. **CLI Flags** (highest priority)
2. **Repository Config**: `.multigit/config.toml`
3. **User Config**: `~/.config/multigit/config.toml`
4. **Defaults** (lowest priority)

### Example Configuration

Create `~/.config/multigit/config.toml`:

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
auth_backend = "keyring"
audit_log = true

[remotes.github]
username = "myusername"
enabled = true
provider = "github"
priority = 0

[remotes.gitlab]
username = "myusername"
api_url = "https://gitlab.com"
enabled = true
provider = "gitlab"
priority = 1
```

### Configuration Options

#### Settings

- `default_branch`: Default branch name (default: "main")
- `parallel_push`: Enable parallel operations (default: true)
- `max_parallel`: Maximum parallel operations (default: 4)
- `colored_output`: Enable colored terminal output (default: true)

#### Sync

- `auto_sync`: Enable automatic synchronization (default: false)
- `strategy`: Sync strategy - "fast-forward", "merge", "rebase", "force"
- `detect_conflicts`: Detect conflicts before syncing (default: true)
- `primary_source`: Primary remote for conflict resolution

#### Security

- `auth_backend`: "keyring", "encrypted-file", or "environment"
- `audit_log`: Enable audit logging (default: false)
- `ssh_agent`: Use SSH agent (default: true)

## Remote Management

### Add a Remote

```bash
multigit remote add <provider> <username>
```

**Supported providers**: github, gitlab, bitbucket, codeberg, gitea

**Interactive mode** (recommended):
```bash
multigit remote add github username
# Prompts for token with help text
```

**With environment variable**:
```bash
export MULTIGIT_GITHUB_TOKEN="ghp_..."
multigit remote add github username
```

### List Remotes

```bash
# Simple list
multigit remote list

# Detailed view
multigit remote list --detailed
```

### Test Connection

```bash
# Test specific remote
multigit remote test github

# Test all remotes
multigit remote test --all
```

### Update Credentials

```bash
multigit remote update github
```

### Remove a Remote

```bash
multigit remote remove bitbucket
```

## Synchronization

### Push to All Remotes

```bash
# Push current branch
multigit push

# Push specific branch
multigit push --branch main

# Force push (use with caution!)
multigit push --force
```

### Pull from Primary Remote

```bash
# Pull from configured primary
multigit pull

# Pull from specific remote
multigit pull --from github
```

### Fetch from Remotes

```bash
# Fetch from all remotes
multigit fetch --all

# Fetch from specific remotes
multigit fetch github gitlab
```

### Full Synchronization

```bash
# Interactive sync with conflict detection
multigit sync

# Force sync (skip conflict detection)
multigit sync --force
```

## Conflict Resolution

### Detect Conflicts

```bash
multigit conflict list
```

Output example:
```
🔍 Checking for conflicts on branch 'main'...

  ⚠ CONFLICT: Diverged github
      Local: 2 commits ahead
      Remote: 3 commits ahead
      ⚠️  Branches have diverged - manual resolution required

  ✓ In sync gitlab
  ✓ In sync bitbucket
```

### Resolve Conflicts

```bash
# Interactive resolution
multigit conflict resolve

# With specific strategy
multigit conflict resolve --strategy ours
multigit conflict resolve --strategy theirs
multigit conflict resolve --strategy primary
```

### Set Primary Remote

```bash
multigit conflict set-primary github
```

The primary remote is used as the source of truth when conflicts occur.

## Daemon Mode

Run MultiGit in the background for automatic synchronization.

### Start Daemon

```bash
# Start with 5-minute interval
multigit daemon start --interval 5

# Start with custom interval
multigit daemon start --interval 30  # 30 minutes
```

**Run in background** (recommended):
```bash
nohup multigit daemon start --interval 5 &
```

### Check Daemon Status

```bash
multigit daemon status
```

Output:
```
📊 Daemon Status:

  Status: ✓ Running
  PID: 12345
  Log file: /home/user/.config/multigit/daemon.log
```

### Stop Daemon

```bash
multigit daemon stop
```

### View Logs

```bash
# View last 50 lines
multigit daemon logs

# View last 100 lines
multigit daemon logs --lines 100
```

## Advanced Usage

### Branch Management

```bash
# List branches
multigit branch list

# Create branch on all remotes
multigit branch create feature-x

# Delete branch from all remotes
multigit branch delete old-feature
```

### Tag Management

```bash
# List tags
multigit tag list

# Create tag on all remotes
multigit tag create v1.0.0 --message "Release v1.0.0"

# Delete tag from all remotes
multigit tag delete old-tag
```

### Repository Creation

```bash
# Create repository on all platforms
multigit create my-new-repo --description "My awesome project"

# Create private repository
multigit create my-private-repo --private

# Interactive creation
multigit create my-repo --interactive
```

### Health Check

```bash
multigit doctor
```

This command:
- Checks repository state
- Verifies remote connectivity
- Validates configuration
- Tests credential storage
- Suggests fixes for issues

### JSON Output

For scripting and automation:

```bash
multigit status --json
multigit remote list --json
multigit conflict list --json
```

## Troubleshooting

### Authentication Issues

**Problem**: "Authentication failed for github"

**Solutions**:
1. Verify your token is valid
2. Check token permissions
3. Update credentials:
   ```bash
   multigit remote update github
   ```

### Remote Connection Failures

**Problem**: "Failed to connect to remote"

**Solutions**:
1. Test connection:
   ```bash
   multigit remote test github
   ```
2. Check network connectivity
3. Verify API URL (for self-hosted instances)

### Conflict Errors

**Problem**: "Branches have diverged"

**Solutions**:
1. Detect conflicts:
   ```bash
   multigit conflict list
   ```
2. Choose resolution strategy:
   - **Fast-forward**: Only if no conflicts
   - **Primary**: Use primary remote as source
   - **Manual**: Resolve manually with git

### Daemon Issues

**Problem**: Daemon not starting

**Solutions**:
1. Check if already running:
   ```bash
   multigit daemon status
   ```
2. Check logs:
   ```bash
   multigit daemon logs
   ```
3. Remove stale PID file:
   ```bash
   rm ~/.config/multigit/daemon.pid
   ```

### Debug Mode

For detailed logging:

```bash
RUST_LOG=debug multigit <command>
```

### Getting Help

```bash
# Command help
multigit --help
multigit remote --help
multigit sync --help

# Version information
multigit --version
```

## Best Practices

### 1. Use Keyring Authentication

Store credentials in your OS keyring for maximum security:

```toml
[security]
auth_backend = "keyring"
```

### 2. Set Up Daemon for Auto-Sync

Keep repositories synchronized automatically:

```bash
multigit daemon start --interval 10
```

### 3. Regular Health Checks

Run periodic health checks:

```bash
multigit doctor
```

### 4. Configure Primary Remote

Set a primary remote for conflict resolution:

```bash
multigit conflict set-primary github
```

### 5. Use Specific Branches

For production workflows, push specific branches:

```bash
multigit push --branch main
```

### 6. Test Before Production

Always test remotes before critical operations:

```bash
multigit remote test --all
```

## Advanced Configuration

### Per-Remote SSH Keys

```toml
[remotes.github]
username = "myuser"
use_ssh = true
priority = 0

[remotes.gitlab]
username = "myuser"
use_ssh = false
priority = 1
```

### Branch Filtering

```toml
[sync]
include_branches = ["main", "develop"]
exclude_branches = ["temp/*", "wip/*"]
```

### Custom Sync Strategies

```toml
[sync]
strategy = "rebase"  # Always rebase
primary_source = "github"  # Use github as truth
```

---

For more information, visit:
- 📖 [Documentation](https://docs.multigit.dev)
- 💬 [Discussions](https://github.com/TIVerse/multigit/discussions)
- 🐛 [Issues](https://github.com/TIVerse/multigit/issues)
