# MultiGit Build & Installation Scripts

This directory contains scripts for building and installing MultiGit across different platforms.

## 📦 Quick Installation

### Linux & macOS
```bash
# Build and install in one command
make install

# Or user-level install (no sudo)
make install-user

# Or run the universal script
./scripts/install.sh
```

### Windows

**Using PowerShell (Recommended):**
```powershell
# Build first
cargo build --release

# Then install
.\scripts\install.ps1
```

**Using Git Bash / WSL:**
```bash
make install
# or
./scripts/install-windows.sh
```

---

## 🔨 Build Scripts

### `build-release.sh`
Builds optimized binaries for all supported platforms and creates distribution packages.

**Supported Platforms:**
- Linux x86_64 (GNU libc)
- Linux x86_64 (MUSL - static)
- macOS x86_64 (Intel)
- macOS ARM64 (Apple Silicon)
- Windows x86_64

**Usage:**
```bash
# Via Make
make dist

# Or directly
./scripts/build-release.sh
```

**Output:**
Creates `dist/` directory with:
- `multigit-{version}-{os}-{arch}.tar.gz` (Linux/macOS)
- `multigit-{version}-{os}-{arch}.zip` (Windows)
- `SHA256SUMS` - Checksum file

**Each package includes:**
- `multigit` and `mg` binaries
- `README.md`
- `LICENSE`
- `CHANGELOG.md`
- `install.sh` / `install.bat` - Platform-specific installer
- `uninstall.sh` / `uninstall.bat` - Uninstaller

---

## 💾 Installation Scripts

### `install.sh`
Universal installation script that works on Linux, macOS, and Windows (Git Bash/WSL).

**Features:**
- Auto-detects OS and architecture
- Chooses appropriate install directory
- Handles permissions automatically
- Verifies PATH configuration

**Install Locations:**
- **Linux/macOS (system):** `/usr/local/bin` (requires sudo)
- **Linux/macOS (user):** `~/.local/bin` (no sudo)
- **Windows:** `%USERPROFILE%\.cargo\bin`

**Usage:**
```bash
# Build first
cargo build --release

# Then install
./scripts/install.sh
```

---

### `install.ps1`
PowerShell installation script for Windows users.

**Features:**
- Native PowerShell experience
- Colored output
- PATH verification
- Helpful instructions

**Usage:**
```powershell
# In PowerShell
cargo build --release
.\scripts\install.ps1
```

---

### `install-windows.sh`
Bash script for installing on Windows via Git Bash, WSL, or MSYS2.

**Usage:**
```bash
cargo build --release
./scripts/install-windows.sh
```

---

## 🎯 Cross-Compilation

### Prerequisites

Install cross-compilation targets:

```bash
# Linux targets
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl

# macOS targets
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Windows target
rustup target add x86_64-pc-windows-msvc
```

### For Linux → Windows cross-compilation:

Install MinGW:
```bash
# Ubuntu/Debian
sudo apt-get install mingw-w64

# macOS
brew install mingw-w64
```

### For Cross-compiling to macOS:

Requires macOS SDK. On Linux, you can use:
```bash
# Install osxcross
git clone https://github.com/tpoechtrager/osxcross
cd osxcross
# Follow the README to set up
```

---

## 📋 Build Commands

### Via Makefile

```bash
# Build for current platform
make release

# Build for all platforms
make build-all-targets

# Create distribution packages
make dist

# Install system-wide
make install

# Install for current user
make install-user

# Install on specific OS
make install-linux
make install-macos
make install-windows
```

### Direct cargo commands

```bash
# Current platform
cargo build --release

# Specific target
cargo build --release --target x86_64-unknown-linux-musl

# All binaries
cargo build --release --bin multigit --bin mg
```

---

## 🗑️ Uninstallation

### Linux & macOS

```bash
# System-wide
make uninstall

# User installation
make uninstall-user

# Or manually
sudo rm /usr/local/bin/multigit /usr/local/bin/mg
rm ~/.local/bin/multigit ~/.local/bin/mg
```

### Windows

**PowerShell:**
```powershell
Remove-Item "$env:USERPROFILE\.cargo\bin\multigit.exe"
Remove-Item "$env:USERPROFILE\.cargo\bin\mg.exe"
```

**Git Bash:**
```bash
rm ~/.cargo/bin/multigit.exe ~/.cargo/bin/mg.exe
```

---

## 🔍 Verification

After installation, verify:

```bash
# Check installation
which multigit
which mg

# Check versions
multigit --version
mg --version

# Test functionality
mg --help
```

---

## 📦 Distribution Package Structure

```
multigit-1.1.0-linux-x86_64/
├── multigit            # Main binary
├── mg                  # Short alias
├── README.md           # Documentation
├── LICENSE             # License file
├── CHANGELOG.md        # Version history
├── install.sh          # Installer
└── uninstall.sh        # Uninstaller
```

---

## 🚀 CI/CD Integration

These scripts are designed to work with CI/CD pipelines:

```yaml
# GitHub Actions example
- name: Build release packages
  run: ./scripts/build-release.sh

- name: Upload artifacts
  uses: actions/upload-artifact@v3
  with:
    name: release-packages
    path: dist/*
```

---

## ⚙️ Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `INSTALL_PATH` | Installation directory | OS-dependent |
| `CARGO_BUILD_TARGET` | Rust target triple | Current platform |
| `RUST_LOG` | Log level | `info` |

---

## 🐛 Troubleshooting

### Permission denied
```bash
# Make scripts executable
chmod +x scripts/*.sh

# Or use bash directly
bash scripts/install.sh
```

### Target not found
```bash
# Install missing target
rustup target add <target-triple>
```

### Windows PATH issues
```powershell
# Check PATH
$env:PATH -split ';'

# Add to PATH permanently
[Environment]::SetEnvironmentVariable('Path', $env:Path + ';C:\path\to\bin', 'User')
```

---

## 📝 Notes

- All scripts include error handling (`set -e`)
- Scripts are idempotent (safe to run multiple times)
- User installations don't require sudo/admin privileges
- Both `multigit` and `mg` binaries are always installed together
- Distribution packages include platform-specific installers

---

## 🔗 Related Commands

```bash
# Clean build artifacts
make clean

# Run tests before release
make pre-release

# Publish to crates.io
make publish

# Full CI simulation
make ci-full
```

---

For more information, see the main [README.md](../README.md) or run `make help`.
