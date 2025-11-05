# Conventional Commit Helper Guide

## Quick Start

```bash
mg cc              # Start the interactive commit helper
# or
multigit cc        # Same thing
# or  
mg commit          # Alternative alias
```

## What It Does

The conventional commit helper (`mg cc`) is an interactive tool that guides you through creating well-formatted conventional commits with proper structure and metadata.

## Features

### 1. **File Selection**
- **All files**: Quickly stage all modified files
- **Select individually**: Choose specific files with multi-select
- Automatically excludes `.gitignore` files
- Shows file status (new, modified, deleted, renamed)

### 2. **Commit Type Selection**
Choose from standard conventional commit types with visual indicators:
- ✨ `feat` - A new feature
- 🐛 `fix` - A bug fix
- 📚 `docs` - Documentation only changes
- 💎 `style` - Code style changes (formatting, semicolons, etc.)
- ♻️  `refactor` - Code refactoring without changing functionality
- ⚡ `perf` - Performance improvements
- ✅ `test` - Adding or updating tests
- 🔨 `build` - Build system or external dependencies
- 👷 `ci` - CI/CD configuration changes
- 🔧 `chore` - Other changes that don't modify src or test files
- ⏪ `revert` - Revert a previous commit

### 3. **Smart Scope Detection**
Automatically detects scopes from your changed files:
- Changed `src/cli/commands/push.rs` → suggests `cli` scope
- Changed `src/core/auth.rs` → suggests `core` or `auth` scope
- Changed `README.md` → suggests `docs` scope
- Changed `Cargo.toml` → suggests `build` scope
- Changed files in `tests/` → suggests `test` scope

You can also:
- Choose "(no scope)" for commits without a scope
- Select from common project scopes
- Enter a custom scope

### 4. **Description & Body**
- Short description with validation:
  - Must not be empty
  - Should be 72 characters or less
  - Should start with lowercase (conventional style)
  - Should use imperative mood ("add" not "added")
- Optional long description (body) via your `$EDITOR`

### 5. **Breaking Changes**
- Mark commits as breaking changes with `!` suffix
- Automatically adds `BREAKING CHANGE:` footer

### 6. **Issue References**
- Add footer for issue references
- Examples: `Closes #123`, `Refs #456`, `Fixes #789`

### 7. **Preview & Edit**
- Preview the complete commit message before committing
- Option to edit in your `$EDITOR` if you want to make changes
- Confirm or cancel before committing

## Workflow Example

```bash
# 1. Make some changes to your code
vim src/cli/commands/push.rs
vim src/core/sync_manager.rs

# 2. Run the interactive commit helper
mg cc

# 3. Follow the prompts:
#    - Select files: Choose "All files" or "Select individually"
#    - Select type: Choose "feat" for new feature
#    - Select scope: Choose "cli" (detected from files)
#    - Enter description: "add remote filter support to push command"
#    - Add body?: No (or Yes if you want detailed description)
#    - Breaking change?: No
#    - Footer: "Closes #42"

# 4. Preview the commit message:
# ─────────────────────────────────────
# feat(cli): add remote filter support to push command
#
# Closes #42
# ─────────────────────────────────────

# 5. Confirm: Yes

# ✅ Commit created successfully!
```

## Output Format

The tool creates commits following the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope][!]: <description>

[optional body]

[optional footer(s)]
```

### Examples

**Simple commit:**
```
feat(api): add rate limiting support
```

**With scope and body:**
```
fix(auth): resolve keyring access on macOS

The keyring library was failing to access the system keychain
on recent macOS versions. Updated to use the new API.
```

**Breaking change:**
```
refactor(core)!: change config file format

BREAKING CHANGE: Configuration files now use YAML instead of TOML.
Migration guide available in MIGRATION.md.

Closes #123
```

**Multiple footers:**
```
feat(sync): add parallel fetch operations

Refs #45
Refs #67
```

## Tips

1. **Use imperative mood**: Write "add feature" not "added feature" or "adds feature"
2. **Keep it short**: First line should be 72 characters or less
3. **Be specific**: Good: "fix login validation" Bad: "fix bug"
4. **Use scopes**: Helps organize changelog and understand impact
5. **Mark breaking changes**: Always use `!` and `BREAKING CHANGE:` footer

## Common Scopes in MultiGit

Based on the project structure, common scopes include:
- `cli` - Command-line interface
- `core` - Core functionality (sync, auth, config)
- `git` - Git operations layer
- `providers` - Provider implementations
- `api` - API client functionality
- `daemon` - Background daemon
- `ui` - User interface components
- `utils` - Utility functions
- `config` - Configuration management
- `auth` - Authentication
- `sync` - Synchronization logic
- `health` - Health checking
- `error` - Error handling
- `docs` - Documentation
- `test` - Tests
- `build` - Build configuration

## Keyboard Shortcuts

During file selection:
- **Space** - Toggle file selection
- **Enter** - Confirm selection
- **↑/↓** - Navigate options

During type/scope selection:
- **↑/↓** - Navigate options
- **Enter** - Select option

## Integration with Git

The tool uses standard `git add` and `git commit` commands under the hood, so it works seamlessly with your existing Git workflow.

## Why Conventional Commits?

1. **Automatic Changelog**: Generate changelogs from commit messages
2. **Semantic Versioning**: Determine version bumps automatically
3. **Better History**: Understand changes at a glance
4. **Team Communication**: Clear, consistent commit messages
5. **Tool Integration**: Many tools support conventional commits

## See Also

- [Conventional Commits Specification](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- MultiGit CHANGELOG.md for examples
