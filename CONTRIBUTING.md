# Contributing to MultiGit

Thank you for your interest in contributing to MultiGit! This guide will help you get started.

## 🎯 Ways to Contribute

- 🐛 Report bugs and issues
- 💡 Suggest new features
- 📝 Improve documentation
- 🔧 Submit bug fixes
- ✨ Implement new features
- 🧪 Add tests
- 🌍 Translate documentation

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (install from [rust-lang.org](https://rust-lang.org))
- Git
- A GitHub/GitLab/Bitbucket account for testing

### Development Setup

```bash
# 1. Fork and clone the repository
git clone https://github.com/TIVerse/multigit.git
cd multigit

# 2. Build the project
cargo build

# 3. Run tests
cargo test

# 4. Run with debug logging
RUST_LOG=debug cargo run -- --help

# 5. Format code
cargo fmt

# 6. Lint code
cargo clippy -- -D warnings
```

## 📝 Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 2. Make Your Changes

Follow these guidelines:

#### Code Style
- Use `cargo fmt` to format code
- Follow Rust naming conventions
- Add rustdoc comments for public APIs
- Write idiomatic Rust code

#### Testing
- Add tests for new features
- Update existing tests if behavior changes
- Ensure all tests pass: `cargo test`
- Aim for 70%+ code coverage

#### Documentation
- Update README.md if needed
- Add rustdoc comments
- Update CHANGELOG.md
- Create examples for new features

### 3. Commit Your Changes

Use conventional commit messages:

```bash
git commit -m "feat: add workspace management"
git commit -m "fix: resolve daemon PID file race condition"
git commit -m "docs: update configuration guide"
git commit -m "test: add conflict resolution tests"
```

**Commit types:**
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `test:` - Tests
- `refactor:` - Code refactoring
- `perf:` - Performance improvement
- `chore:` - Maintenance

### 4. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then open a Pull Request on GitHub with:
- Clear description of changes
- Link to related issues
- Screenshots (if UI changes)
- Test results

## 🏗️ Project Structure

```
multigit/
├── src/
│   ├── api/          # API client utilities
│   ├── cli/          # CLI commands and interactive prompts
│   ├── core/         # Core functionality (config, auth, sync)
│   ├── daemon/       # Background daemon service
│   ├── git/          # Git operations wrapper
│   ├── models/       # Data models
│   ├── providers/    # Git hosting provider implementations
│   ├── security/     # Security and credential management
│   ├── ui/           # User interface components
│   └── utils/        # Utility functions
├── tests/
│   ├── unit/         # Unit tests
│   ├── integration/  # Integration tests
│   └── fixtures/     # Test fixtures and mocks
├── examples/         # Example programs
└── docs/             # Documentation
```

## 🧪 Testing Guidelines

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test test_runner
```

### Writing Tests

```rust
#[test]
fn test_feature() {
    // Arrange
    let config = Config::default();
    
    // Act
    let result = config.enabled_remotes();
    
    // Assert
    assert_eq!(result.len(), 0);
}

#[tokio::test]
async fn test_async_feature() {
    let result = some_async_function().await;
    assert!(result.is_ok());
}
```

## 🔌 Adding a New Provider

To add support for a new Git hosting platform:

1. Create `src/providers/yourprovider.rs`
2. Implement the `Provider` trait
3. Add authentication handling
4. Implement API methods
5. Add to `src/providers/mod.rs`
6. Write tests
7. Update documentation

Example template:

```rust
use crate::providers::traits::Provider;
use async_trait::async_trait;

pub struct YourProvider {
    client: reqwest::Client,
    token: String,
    username: String,
}

impl YourProvider {
    pub fn new(token: String, username: String) -> Result<Self> {
        Ok(Self {
            client: reqwest::Client::new(),
            token,
            username,
        })
    }
}

#[async_trait]
impl Provider for YourProvider {
    fn name(&self) -> &str {
        "yourprovider"
    }
    
    async fn test_connection(&self) -> anyhow::Result<bool> {
        // Implementation
    }
    
    // ... implement other trait methods
}
```

## 📚 Documentation

### Rustdoc Comments

```rust
/// Brief description of the function
///
/// More detailed explanation of what the function does,
/// its parameters, and return value.
///
/// # Arguments
///
/// * `name` - The name of the repository
/// * `private` - Whether the repository should be private
///
/// # Returns
///
/// Returns the created repository or an error
///
/// # Example
///
/// ```
/// let repo = create_repo("my-repo", false)?;
/// ```
pub fn create_repo(name: &str, private: bool) -> Result<Repository> {
    // Implementation
}
```

## 🐛 Reporting Bugs

When reporting bugs, please include:

1. **Description**: Clear description of the issue
2. **Steps to Reproduce**: Exact steps to trigger the bug
3. **Expected Behavior**: What should happen
4. **Actual Behavior**: What actually happens
5. **Environment**:
   - OS and version
   - Rust version (`rustc --version`)
   - MultiGit version (`multigit --version`)
6. **Logs**: Relevant log output with `RUST_LOG=debug`

## 💡 Feature Requests

For feature requests, please describe:

1. **Use Case**: Why this feature is needed
2. **Proposed Solution**: How you envision it working
3. **Alternatives**: Other approaches you've considered
4. **Additional Context**: Any other relevant information

## 🎯 Pull Request Checklist

Before submitting a PR, ensure:

- [ ] Code follows Rust conventions
- [ ] `cargo fmt` has been run
- [ ] `cargo clippy` passes without warnings
- [ ] All tests pass (`cargo test`)
- [ ] New tests added for new functionality
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Commit messages follow conventions
- [ ] PR description is clear and complete

## 🤝 Code of Conduct

### Our Standards

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions
- Accept responsibility and learn from mistakes

### Unacceptable Behavior

- Harassment or discrimination
- Trolling or insulting comments
- Personal or political attacks
- Publishing private information

## 📞 Getting Help

- 💬 [GitHub Discussions](https://github.com/TIVerse/multigit/discussions)
- 🐛 [Issue Tracker](https://github.com/TIVerse/multigit/issues)
- 📧 Email: contribute@tiverse.dev

## 📄 License

By contributing to MultiGit, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for making MultiGit better! 🎉**
