#!/bin/bash
# Windows installation script for MultiGit
# Can be run from Git Bash, WSL, or MSYS2

set -e

echo "🪟 MultiGit Windows Installation"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Detect Windows environment
if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    INSTALL_DIR="$HOME/.cargo/bin"
elif [[ -n "$USERPROFILE" ]]; then
    INSTALL_DIR="$(cygpath "$USERPROFILE")/.cargo/bin"
else
    echo "Error: Could not detect Windows user profile"
    exit 1
fi

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Copy binaries
echo "Installing binaries to: $INSTALL_DIR"
cp -v target/release/multigit.exe "$INSTALL_DIR/"
cp -v target/release/mg.exe "$INSTALL_DIR/"

echo ""
echo "✅ Installation complete!"
echo ""
echo "Verify installation:"
echo "  multigit.exe --version"
echo "  mg.exe --version"
echo ""
echo "💡 Make sure $INSTALL_DIR is in your PATH"
echo ""
echo "To add to PATH permanently:"
echo '  setx PATH "%PATH%;%USERPROFILE%\.cargo\bin"'
echo ""
