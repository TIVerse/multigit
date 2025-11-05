#!/bin/bash
# Universal installation script for MultiGit
# Works on Linux, macOS, and Windows (Git Bash/WSL)

set -e

VERSION="1.1.0"
PROJECT="multigit"
BINARY_ALIAS="mg"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 MultiGit Universal Installer"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)     OS=linux;;
        Darwin*)    OS=macos;;
        MINGW*|MSYS*|CYGWIN*) OS=windows;;
        *)          OS=unknown;;
    esac
    echo "Detected OS: $OS"
}

# Detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64)     ARCH=x86_64;;
        aarch64|arm64) ARCH=arm64;;
        *)          ARCH=unknown;;
    esac
    echo "Architecture: $ARCH"
}

# Determine install directory
get_install_dir() {
    if [ "$OS" = "windows" ]; then
        if [ -n "$USERPROFILE" ]; then
            INSTALL_DIR="$(cygpath "$USERPROFILE")/.cargo/bin"
        else
            INSTALL_DIR="$HOME/.cargo/bin"
        fi
    elif [ -w "/usr/local/bin" ]; then
        INSTALL_DIR="/usr/local/bin"
        NEED_SUDO=false
    elif [ "$(id -u)" = "0" ]; then
        INSTALL_DIR="/usr/local/bin"
        NEED_SUDO=false
    else
        INSTALL_DIR="$HOME/.local/bin"
        NEED_SUDO=false
    fi
    
    echo "Install directory: $INSTALL_DIR"
}

# Download or use local binaries
install_binaries() {
    echo ""
    echo "📦 Installing binaries..."
    
    # Check if we have local build
    if [ -f "target/release/$PROJECT" ] || [ -f "target/release/${PROJECT}.exe" ]; then
        echo "Using local build..."
        
        if [ "$OS" = "windows" ]; then
            cp target/release/${PROJECT}.exe "$INSTALL_DIR/"
            cp target/release/${BINARY_ALIAS}.exe "$INSTALL_DIR/"
        else
            if [ "$NEED_SUDO" = "true" ]; then
                sudo install -m755 target/release/$PROJECT "$INSTALL_DIR/$PROJECT"
                sudo install -m755 target/release/$BINARY_ALIAS "$INSTALL_DIR/$BINARY_ALIAS"
            else
                install -m755 target/release/$PROJECT "$INSTALL_DIR/$PROJECT"
                install -m755 target/release/$BINARY_ALIAS "$INSTALL_DIR/$BINARY_ALIAS"
            fi
        fi
    else
        echo "Error: No local build found."
        echo "Please run 'cargo build --release' first"
        exit 1
    fi
}

# Create install directory
create_install_dir() {
    if [ ! -d "$INSTALL_DIR" ]; then
        echo "Creating $INSTALL_DIR..."
        mkdir -p "$INSTALL_DIR"
    fi
}

# Main installation
main() {
    detect_os
    detect_arch
    get_install_dir
    create_install_dir
    install_binaries
    
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "✅ Installation Complete!"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "Installed to: $INSTALL_DIR"
    echo ""
    echo "Verify installation:"
    if [ "$OS" = "windows" ]; then
        echo "  ${PROJECT}.exe --version"
        echo "  ${BINARY_ALIAS}.exe --version"
    else
        echo "  $PROJECT --version"
        echo "  $BINARY_ALIAS --version"
    fi
    echo ""
    
    # Check if in PATH
    if ! command -v $PROJECT &> /dev/null; then
        echo "⚠️  Warning: $INSTALL_DIR is not in your PATH"
        echo ""
        echo "Add to PATH:"
        if [ "$OS" = "linux" ]; then
            echo "  echo 'export PATH=\$PATH:$INSTALL_DIR' >> ~/.bashrc"
            echo "  source ~/.bashrc"
        elif [ "$OS" = "macos" ]; then
            echo "  echo 'export PATH=\$PATH:$INSTALL_DIR' >> ~/.zshrc"
            echo "  source ~/.zshrc"
        elif [ "$OS" = "windows" ]; then
            echo '  setx PATH "%PATH%;%USERPROFILE%\.cargo\bin"'
        fi
        echo ""
    else
        echo "Get started:"
        echo "  $BINARY_ALIAS --help"
        echo "  $BINARY_ALIAS init"
        echo ""
    fi
}

main
