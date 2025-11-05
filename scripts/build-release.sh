#!/bin/bash
# Cross-platform release build script for MultiGit
# Generates optimized binaries for Linux, macOS, and Windows

set -e

VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
DIST_DIR="dist"
PROJECT_NAME="multigit"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 MultiGit Cross-Platform Release Builder"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Version: $VERSION"
echo "Output:  $DIST_DIR/"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Clean and create dist directory
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Function to build for a target
build_target() {
    local target=$1
    local os_name=$2
    local arch=$3
    local ext=$4
    
    echo ""
    echo "📦 Building for $os_name ($arch)..."
    echo "   Target: $target"
    
    # Build
    cargo build --release --target "$target"
    
    # Create package directory
    local pkg_name="${PROJECT_NAME}-${VERSION}-${os_name}-${arch}"
    local pkg_dir="$DIST_DIR/$pkg_name"
    mkdir -p "$pkg_dir"
    
    # Copy binaries
    cp "target/$target/release/${PROJECT_NAME}${ext}" "$pkg_dir/"
    cp "target/$target/release/mg${ext}" "$pkg_dir/"
    
    # Copy documentation
    cp README.md "$pkg_dir/"
    cp LICENSE "$pkg_dir/" 2>/dev/null || echo "# License TBD" > "$pkg_dir/LICENSE"
    cp CHANGELOG.md "$pkg_dir/" 2>/dev/null || true
    
    # Create install script
    if [ -z "$ext" ]; then
        # Unix-like systems
        cat > "$pkg_dir/install.sh" << 'INSTALL_SCRIPT'
#!/bin/bash
set -e

echo "Installing MultiGit..."

# Detect install location
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
elif [ -w "$HOME/.local/bin" ]; then
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
else
    echo "Error: No writable install directory found"
    echo "Try running: sudo ./install.sh"
    exit 1
fi

# Install binaries
install -m755 multigit "$INSTALL_DIR/multigit"
install -m755 mg "$INSTALL_DIR/mg"

echo "✅ Installed to $INSTALL_DIR"
echo ""
echo "Verify installation:"
echo "  multigit --version"
echo "  mg --version"
echo ""
echo "Get started:"
echo "  mg --help"
INSTALL_SCRIPT
        chmod +x "$pkg_dir/install.sh"
        
        # Create uninstall script
        cat > "$pkg_dir/uninstall.sh" << 'UNINSTALL_SCRIPT'
#!/bin/bash
set -e

echo "Uninstalling MultiGit..."

# Try system location first
if [ -f "/usr/local/bin/multigit" ]; then
    sudo rm -f /usr/local/bin/multigit /usr/local/bin/mg
    echo "✅ Removed from /usr/local/bin"
fi

# Try user location
if [ -f "$HOME/.local/bin/multigit" ]; then
    rm -f "$HOME/.local/bin/multigit" "$HOME/.local/bin/mg"
    echo "✅ Removed from ~/.local/bin"
fi

echo "Uninstall complete"
UNINSTALL_SCRIPT
        chmod +x "$pkg_dir/uninstall.sh"
    else
        # Windows
        cat > "$pkg_dir/install.bat" << 'INSTALL_BATCH'
@echo off
echo Installing MultiGit...

REM Copy to user's cargo bin directory
set INSTALL_DIR=%USERPROFILE%\.cargo\bin
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

copy /Y multigit.exe "%INSTALL_DIR%\multigit.exe"
copy /Y mg.exe "%INSTALL_DIR%\mg.exe"

echo.
echo ✅ Installed to %INSTALL_DIR%
echo.
echo Verify installation:
echo   multigit --version
echo   mg --version
echo.
echo Get started:
echo   mg --help
pause
INSTALL_BATCH

        cat > "$pkg_dir/uninstall.bat" << 'UNINSTALL_BATCH'
@echo off
echo Uninstalling MultiGit...

set INSTALL_DIR=%USERPROFILE%\.cargo\bin

del /F "%INSTALL_DIR%\multigit.exe" 2>nul
del /F "%INSTALL_DIR%\mg.exe" 2>nul

echo.
echo ✅ Uninstall complete
pause
UNINSTALL_BATCH
    fi
    
    # Create archive
    cd "$DIST_DIR"
    if [ -z "$ext" ]; then
        # tar.gz for Unix
        tar -czf "${pkg_name}.tar.gz" "$pkg_name"
        echo "   ✅ Created ${pkg_name}.tar.gz"
    else
        # zip for Windows
        zip -qr "${pkg_name}.zip" "$pkg_name"
        echo "   ✅ Created ${pkg_name}.zip"
    fi
    cd ..
    
    # Remove temp directory
    rm -rf "$pkg_dir"
}

# Build for all platforms
echo ""
echo "🔨 Starting builds..."

# Linux x86_64 (GNU libc)
build_target "x86_64-unknown-linux-gnu" "linux" "x86_64" ""

# Linux x86_64 (MUSL - static binary)
build_target "x86_64-unknown-linux-musl" "linux" "x86_64-musl" ""

# macOS x86_64 (Intel)
build_target "x86_64-apple-darwin" "macos" "x86_64" ""

# macOS ARM64 (Apple Silicon)
if rustup target list | grep -q "aarch64-apple-darwin (installed)"; then
    build_target "aarch64-apple-darwin" "macos" "arm64" ""
else
    echo "⚠️  Skipping macOS ARM64 (target not installed)"
    echo "   Install with: rustup target add aarch64-apple-darwin"
fi

# Windows x86_64
if rustup target list | grep -q "x86_64-pc-windows-msvc (installed)"; then
    build_target "x86_64-pc-windows-msvc" "windows" "x86_64" ".exe"
elif rustup target list | grep -q "x86_64-pc-windows-gnu (installed)"; then
    build_target "x86_64-pc-windows-gnu" "windows" "x86_64" ".exe"
else
    echo "⚠️  Skipping Windows (target not installed)"
    echo "   Install with: rustup target add x86_64-pc-windows-msvc"
fi

# Generate checksums
echo ""
echo "🔐 Generating checksums..."
cd "$DIST_DIR"
sha256sum *.tar.gz *.zip 2>/dev/null > SHA256SUMS || shasum -a 256 *.tar.gz *.zip > SHA256SUMS
cd ..

# Summary
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Build Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📦 Release packages:"
ls -lh "$DIST_DIR"/*.tar.gz "$DIST_DIR"/*.zip 2>/dev/null
echo ""
echo "🔐 Checksums: $DIST_DIR/SHA256SUMS"
echo ""
echo "📤 Ready for distribution!"
echo ""
