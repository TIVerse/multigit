# MultiGit v2.0.0 Release Notes

**Release Date**: November 6, 2025

## 🎉 Major Release: v2.0.0

This major release brings significant enhancements to developer experience, code quality, and operational reliability. We've added powerful new features while fixing critical bugs and improving the overall architecture.

---

## ✨ New Features

### Conventional Commit Helper (`mg cc`)
Interactive tool that guides you through creating well-formatted conventional commits:
- **Smart file selection**: Choose "All files" or select individually
- **Type selection**: feat, fix, docs, style, refactor, perf, test, build, ci, chore
- **Auto-detected scopes**: Intelligently suggests scopes from your file paths
- **Breaking changes**: Mark commits with breaking changes
- **Issue references**: Link commits to issues (e.g., "Closes #123")
- **Preview & edit**: Review before committing with option to edit in your editor
- Available as `mg cc`, `multigit cc`, or `multigit commit`

### Short Command Alias (`mg`)
Added `mg` as a shorter, faster alternative to `multigit`:
- Both binaries built from the same source
- Identical functionality
- Use `mg init`, `mg sync`, `mg cc` for faster typing
- Perfect for interactive use while `multigit` is great for scripts

### Unified Provider Factory
Centralized provider creation logic in `src/providers/factory.rs`:
- Eliminates ~80 lines of code duplication
- Single source of truth for supported providers
- Easier to add new providers
- Consistent provider instantiation across the codebase

### Remote Health Checks in `multigit doctor`
Real connectivity testing for each configured remote:
- Tests remotes with `git ls-remote` equivalent
- 10-second timeout for health checks
- Categorized errors: authentication, network, timeout
- Actionable troubleshooting information

---

## 🐛 Bug Fixes

### CLI Flag Handling
Fixed ignored and missing command-line arguments:
- ✅ `multigit sync --dry-run` now works correctly
- ✅ `multigit sync --branch <name>` properly syncs specific branch
- ✅ `multigit push --remotes <list>` correctly filters remotes
- All commands now properly receive CLI parameters

### Network Error Retryability
Fixed `MultiGitError::network()` helper:
- Added `NetworkMessage` variant for custom network errors
- Network errors properly marked as retryable
- Consistent error handling across the codebase

### Fetch Metrics Accuracy
Fixed commit counting in fetch operations:
- Changed from comparing HEAD to comparing remote refs
- Accurately reports number of updated refs after fetch
- Better visibility into sync operations

### Push Timeout Monitoring
Enhanced timeout handling during push:
- Added `pack_progress` callback for pack generation phase
- Improved timeout detection and logging
- Clear error messages when timeouts occur

---

## 🔄 Changes & Improvements

### Configuration Documentation
Enhanced documentation for config scope:
- Clarified `Config::save()` behavior (saves to user config by default)
- Documented `save_repo_config()` for repository-specific settings
- Explained hierarchical config loading (defaults → user → repo)

### Provider Creation Refactor
Refactored to use shared factory pattern:
- `setup.rs` and `remote.rs` now use `providers::factory::create_provider()`
- Reduced code duplication
- Easier maintenance and extension

---

## 🗑️ Deprecated

### Alternative CLI Parser
Marked `src/cli/parser.rs` as unused:
- Added documentation warning for contributors
- Preserved for historical reference
- Active CLI definition remains in `src/main.rs`

---

## 📦 Installation

### From crates.io
```bash
cargo install multigit
```

### From source
```bash
git clone https://github.com/TIVerse/multigit.git
cd multigit
cargo build --release
```

### Verify installation
```bash
multigit --version  # Should show v2.0.0
mg --version        # Should show v2.0.0
```

---

## 🚀 Quick Start (New Users)

```bash
# Initialize in your repository
mg init

# Add remotes
mg remote add github your-username
mg remote add gitlab your-username

# Use the new conventional commit helper
mg cc

# Sync to all remotes
mg sync

# Check status
mg status
```

---

## 🆙 Upgrade Guide (Existing Users)

### From v1.x to v2.0.0

**No breaking changes!** Your existing configuration and workflows will continue to work.

**New capabilities you can try:**
1. Use `mg` instead of `multigit` for faster typing
2. Try the new commit helper: `mg cc`
3. Run health checks: `mg doctor`

**Configuration updates:**
- All existing configs remain compatible
- Consider using `--dry-run` flags now that they work correctly

---

## 🙏 Acknowledgments

Thanks to all contributors and users who reported issues and provided feedback!

---

## 📄 Full Changelog

See [CHANGELOG.md](CHANGELOG.md) for complete details.

---

## 🔗 Links

- **Repository**: https://github.com/TIVerse/multigit
- **Crates.io**: https://crates.io/crates/multigit
- **Documentation**: https://docs.rs/multigit
- **Issues**: https://github.com/TIVerse/multigit/issues
- **Discussions**: https://github.com/TIVerse/multigit/discussions

---

**Made with ❤️ by the TIVerse Team**
