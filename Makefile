# MultiGit Makefile
# Production-ready multi-remote Git synchronization tool
# https://github.com/TIVerse/multigit

.PHONY: help all build release install uninstall clean test check fmt lint doc examples coverage bench run dev

# Variables
BINARY_NAME := multigit
BINARY_ALIAS := mg
CARGO := cargo
TARGET_DIR := target
BUILD_TYPE := release
RUST_LOG ?= info

# Detect OS
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
    OS := linux
    INSTALL_PATH := /usr/local/bin
    EXT :=
endif
ifeq ($(UNAME_S),Darwin)
    OS := macos
    INSTALL_PATH := /usr/local/bin
    EXT :=
endif
ifeq ($(OS),Windows_NT)
    OS := windows
    INSTALL_PATH := $(USERPROFILE)\.cargo\bin
    EXT := .exe
else
    ifeq ($(findstring MINGW,$(UNAME_S)),MINGW)
        OS := windows
        INSTALL_PATH := $(HOME)/.cargo/bin
        EXT := .exe
    endif
endif

# Default target
.DEFAULT_GOAL := help

##@ General

help: ## Display this help message
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

all: clean fmt lint test build ## Run all checks and build

##@ Building

build: ## Build debug binary
	@echo "🔨 Building debug binary..."
	@$(CARGO) build --verbose

release: ## Build optimized release binary
	@echo "🚀 Building optimized release binary..."
	@$(CARGO) build --release --verbose --bin $(BINARY_NAME)
	@$(CARGO) build --release --verbose --bin $(BINARY_ALIAS)
	@echo "✅ Binaries built:"
	@echo "   $(TARGET_DIR)/release/$(BINARY_NAME)$(EXT)"
	@echo "   $(TARGET_DIR)/release/$(BINARY_ALIAS)$(EXT)"

build-all-targets: ## Build for all platforms
	@echo "🌍 Building for all platforms..."
	@echo "Building Linux x86_64 (GNU)..."
	@$(CARGO) build --release --target x86_64-unknown-linux-gnu
	@echo "Building Linux x86_64 (MUSL - static)..."
	@$(CARGO) build --release --target x86_64-unknown-linux-musl
	@echo "Building macOS x86_64..."
	@$(CARGO) build --release --target x86_64-apple-darwin
	@echo "Building macOS ARM64 (Apple Silicon)..."
	@$(CARGO) build --release --target aarch64-apple-darwin
	@echo "Building Windows x86_64..."
	@$(CARGO) build --release --target x86_64-pc-windows-msvc
	@echo "✅ All platform builds completed!"

dist: ## Create distribution packages for all platforms
	@echo "📦 Creating distribution packages..."
	@bash scripts/build-release.sh

check: ## Check code without building
	@echo "🔍 Checking code..."
	@$(CARGO) check --all-targets --all-features

##@ Testing

test: ## Run all tests
	@echo "🧪 Running all tests..."
	@$(CARGO) test --verbose

test-unit: ## Run unit tests only
	@echo "🧪 Running unit tests..."
	@$(CARGO) test --lib --verbose

test-integration: ## Run integration tests only
	@echo "🧪 Running integration tests..."
	@$(CARGO) test --test test_runner --verbose

test-doc: ## Run documentation tests
	@echo "📚 Running doc tests..."
	@$(CARGO) test --doc --verbose

test-nocapture: ## Run tests with output
	@echo "🧪 Running tests with output..."
	@$(CARGO) test --verbose -- --nocapture

test-watch: ## Run tests in watch mode (requires cargo-watch)
	@echo "👀 Watching tests..."
	@$(CARGO) watch -x test

##@ Code Quality

fmt: ## Format code with rustfmt
	@echo "🎨 Formatting code..."
	@$(CARGO) fmt --all

fmt-check: ## Check code formatting
	@echo "🔍 Checking code formatting..."
	@$(CARGO) fmt --all -- --check

lint: ## Run clippy linter
	@echo "🔎 Running clippy..."
	@$(CARGO) clippy --all-targets --all-features -- -D warnings

lint-fix: ## Fix clippy warnings automatically
	@echo "🔧 Fixing clippy warnings..."
	@$(CARGO) clippy --fix --allow-dirty --allow-staged

audit: ## Check for security vulnerabilities
	@echo "🔒 Auditing dependencies..."
	@$(CARGO) audit

outdated: ## Check for outdated dependencies
	@echo "📦 Checking outdated dependencies..."
	@$(CARGO) outdated

##@ Documentation

doc: ## Generate documentation
	@echo "📖 Generating documentation..."
	@$(CARGO) doc --no-deps --all-features

doc-open: ## Generate and open documentation
	@echo "📖 Generating and opening documentation..."
	@$(CARGO) doc --no-deps --all-features --open

doc-private: ## Generate documentation including private items
	@echo "📖 Generating full documentation..."
	@$(CARGO) doc --no-deps --all-features --document-private-items

##@ Examples

examples: ## Run all examples
	@echo "🎯 Running examples..."
	@$(CARGO) run --example basic_usage
	@$(CARGO) run --example scheduler_example
	@$(CARGO) run --example ui_formatting

example-basic: ## Run basic usage example
	@echo "🎯 Running basic usage example..."
	@$(CARGO) run --example basic_usage

example-scheduler: ## Run scheduler example
	@echo "🎯 Running scheduler example..."
	@$(CARGO) run --example scheduler_example

example-ui: ## Run UI formatting example
	@echo "🎯 Running UI formatting example..."
	@$(CARGO) run --example ui_formatting

##@ Coverage

coverage: ## Generate code coverage report
	@echo "📊 Generating coverage report..."
	@$(CARGO) tarpaulin --out Html --output-dir coverage --verbose

coverage-xml: ## Generate XML coverage report
	@echo "📊 Generating XML coverage report..."
	@$(CARGO) tarpaulin --out Xml --output-dir coverage --verbose

coverage-lcov: ## Generate LCOV coverage report
	@echo "📊 Generating LCOV coverage report..."
	@$(CARGO) tarpaulin --out Lcov --output-dir coverage --verbose

coverage-open: ## Generate and open coverage report
	@echo "📊 Generating and opening coverage report..."
	@$(CARGO) tarpaulin --out Html --output-dir coverage --verbose
	@open coverage/index.html || xdg-open coverage/index.html 2>/dev/null

##@ Installation

install: release ## Install binaries to system (Linux/macOS: /usr/local/bin, Windows: cargo bin)
	@echo "📦 Installing $(BINARY_NAME) and $(BINARY_ALIAS) for $(OS)..."
ifeq ($(OS),windows)
	@echo "Installing to $(INSTALL_PATH)..."
	@copy /Y $(TARGET_DIR)\release\$(BINARY_NAME).exe $(INSTALL_PATH)\$(BINARY_NAME).exe
	@copy /Y $(TARGET_DIR)\release\$(BINARY_ALIAS).exe $(INSTALL_PATH)\$(BINARY_ALIAS).exe
	@echo "✅ Installed both binaries to $(INSTALL_PATH)"
else
	@echo "Installing to $(INSTALL_PATH)..."
	@sudo install -Dm755 $(TARGET_DIR)/release/$(BINARY_NAME) $(INSTALL_PATH)/$(BINARY_NAME)
	@sudo install -Dm755 $(TARGET_DIR)/release/$(BINARY_ALIAS) $(INSTALL_PATH)/$(BINARY_ALIAS)
	@echo "✅ Installed both binaries to $(INSTALL_PATH)"
endif
	@echo "💡 Run '$(BINARY_NAME) --version' or '$(BINARY_ALIAS) --version' to verify"

install-user: release ## Install binaries to user directory (no sudo required)
	@echo "📦 Installing $(BINARY_NAME) and $(BINARY_ALIAS) to user directory..."
ifeq ($(OS),windows)
	@echo "Installing to $(INSTALL_PATH)..."
	@copy /Y $(TARGET_DIR)\release\$(BINARY_NAME).exe $(INSTALL_PATH)\$(BINARY_NAME).exe
	@copy /Y $(TARGET_DIR)\release\$(BINARY_ALIAS).exe $(INSTALL_PATH)\$(BINARY_ALIAS).exe
else
	@mkdir -p ~/.local/bin
	@install -m755 $(TARGET_DIR)/release/$(BINARY_NAME) ~/.local/bin/$(BINARY_NAME)
	@install -m755 $(TARGET_DIR)/release/$(BINARY_ALIAS) ~/.local/bin/$(BINARY_ALIAS)
	@echo "✅ Installed both binaries to ~/.local/bin"
endif
	@echo "💡 Make sure your user bin directory is in PATH"

install-macos: release ## Install binaries on macOS with proper permissions
	@echo "🍎 Installing for macOS..."
	@sudo install -m755 $(TARGET_DIR)/release/$(BINARY_NAME) /usr/local/bin/$(BINARY_NAME)
	@sudo install -m755 $(TARGET_DIR)/release/$(BINARY_ALIAS) /usr/local/bin/$(BINARY_ALIAS)
	@echo "✅ Installed to /usr/local/bin"
	@which $(BINARY_NAME)
	@which $(BINARY_ALIAS)

install-linux: release ## Install binaries on Linux with proper permissions
	@echo "🐧 Installing for Linux..."
	@sudo install -Dm755 $(TARGET_DIR)/release/$(BINARY_NAME) /usr/local/bin/$(BINARY_NAME)
	@sudo install -Dm755 $(TARGET_DIR)/release/$(BINARY_ALIAS) /usr/local/bin/$(BINARY_ALIAS)
	@echo "✅ Installed to /usr/local/bin"
	@which $(BINARY_NAME)
	@which $(BINARY_ALIAS)

install-windows: release ## Install binaries on Windows
	@echo "🪟 Installing for Windows..."
	@bash scripts/install-windows.sh

uninstall: ## Uninstall binaries from system
	@echo "🗑️  Uninstalling $(BINARY_NAME) and $(BINARY_ALIAS)..."
ifeq ($(OS),windows)
	@del /F $(INSTALL_PATH)\$(BINARY_NAME).exe
	@del /F $(INSTALL_PATH)\$(BINARY_ALIAS).exe
else
	@sudo rm -f $(INSTALL_PATH)/$(BINARY_NAME)
	@sudo rm -f $(INSTALL_PATH)/$(BINARY_ALIAS)
endif
	@echo "✅ Uninstalled both binaries"

uninstall-user: ## Uninstall binaries from user directory
	@echo "🗑️  Uninstalling from user directory..."
ifeq ($(OS),windows)
	@del /F $(INSTALL_PATH)\$(BINARY_NAME).exe
	@del /F $(INSTALL_PATH)\$(BINARY_ALIAS).exe
else
	@rm -f ~/.local/bin/$(BINARY_NAME)
	@rm -f ~/.local/bin/$(BINARY_ALIAS)
endif
	@echo "✅ Uninstalled both binaries"

##@ Development

dev: ## Run development binary with debug logging
	@echo "🚧 Running development build..."
	@RUST_LOG=debug $(CARGO) run -- --help

run: ## Run release binary
	@echo "▶️  Running $(BINARY_NAME)..."
	@$(CARGO) run --release

run-help: ## Show help message
	@$(CARGO) run --release -- --help

run-version: ## Show version
	@$(CARGO) run --release -- --version

watch: ## Watch for changes and rebuild
	@echo "👀 Watching for changes..."
	@$(CARGO) watch -x "build --all-features"

##@ Benchmarking

bench: ## Run benchmarks
	@echo "⚡ Running benchmarks..."
	@$(CARGO) bench

bench-save: ## Run benchmarks and save baseline
	@echo "⚡ Running benchmarks and saving baseline..."
	@$(CARGO) bench -- --save-baseline

##@ Release

pre-release: clean fmt lint test doc ## Run all pre-release checks
	@echo "✅ Pre-release checks completed successfully!"
	@echo "📦 Ready to create release build"

release-build: pre-release release ## Build release with all checks
	@echo "🎉 Release build completed!"

release-tag: ## Create and push a new version tag
	@echo "🏷️  Creating release tag..."
	@read -p "Enter version (e.g., v1.0.0): " version; \
	git tag -a $$version -m "Release $$version"; \
	git push origin $$version

publish-dry: ## Dry run cargo publish (not used - multigit name taken on crates.io)
	@echo "⚠️  Note: 'multigit' name is already taken on crates.io"
	@echo "🚀 Performing dry run publish (for testing only)..."
	@$(CARGO) publish --dry-run --allow-dirty

# Note: multigit name is taken on crates.io, so we don't publish there
# Releases are distributed via GitHub Releases instead
# publish: ## Publish to crates.io
#	@echo "🚀 Publishing to crates.io..."
#	@$(CARGO) publish

##@ Cleaning

clean: ## Remove build artifacts
	@echo "🧹 Cleaning build artifacts..."
	@$(CARGO) clean
	@rm -rf coverage/

clean-all: clean ## Remove all generated files including locks
	@echo "🧹 Deep cleaning..."
	@rm -f Cargo.lock

clean-doc: ## Remove generated documentation
	@echo "🧹 Cleaning documentation..."
	@rm -rf $(TARGET_DIR)/doc

##@ Docker (Optional)

docker-build: ## Build Docker image
	@echo "🐳 Building Docker image..."
	@docker build -t $(BINARY_NAME):latest .

docker-run: ## Run in Docker container
	@echo "🐳 Running in Docker..."
	@docker run -it --rm $(BINARY_NAME):latest

##@ Maintenance

update: ## Update dependencies
	@echo "📦 Updating dependencies..."
	@$(CARGO) update

tree: ## Show dependency tree
	@echo "🌳 Showing dependency tree..."
	@$(CARGO) tree

bloat: ## Analyze binary size
	@echo "📏 Analyzing binary size..."
	@$(CARGO) bloat --release

verify: ## Verify project integrity
	@echo "✅ Verifying project..."
	@$(CARGO) verify-project

fix: ## Auto-fix code issues
	@echo "🔧 Auto-fixing code issues..."
	@$(CARGO) fix --allow-dirty --allow-staged

##@ CI/CD Simulation

ci-test: ## Simulate CI test workflow
	@echo "🤖 Simulating CI test workflow..."
	@$(MAKE) fmt-check
	@$(MAKE) lint
	@$(MAKE) build
	@$(MAKE) test
	@$(MAKE) test-doc
	@echo "✅ CI test simulation passed!"

ci-full: clean ci-test release ## Full CI workflow simulation
	@echo "✅ Full CI simulation completed!"

##@ Information

info: ## Show project information
	@echo "📦 Project Information"
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@echo "Name:        $(BINARY_NAME)"
	@echo "Version:     $$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)"
	@echo "Rust:        $$(rustc --version)"
	@echo "Cargo:       $$(cargo --version)"
	@echo "Target Dir:  $(TARGET_DIR)"
	@echo "Install Dir: $(INSTALL_PATH)"
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

deps: ## Show direct dependencies
	@echo "📦 Direct Dependencies:"
	@$(CARGO) metadata --format-version 1 --no-deps | jq -r '.packages[0].dependencies[] | "  - \(.name) \(.req)"'

lines: ## Count lines of code
	@echo "📊 Lines of Code:"
	@find src -name "*.rs" | xargs wc -l | tail -1 | awk '{print "  Source: " $$1 " lines"}'
	@find tests -name "*.rs" | xargs wc -l | tail -1 | awk '{print "  Tests:  " $$1 " lines"}'

features: ## List available features
	@echo "🎯 Available Features:"
	@grep '\[features\]' Cargo.toml -A 20 | grep -E '^\w+\s*=' | sed 's/=.*//' | sed 's/^/  - /'

##@ Quick Commands

q: fmt lint test ## Quick check: format, lint, and test
	@echo "✅ Quick checks passed!"

qq: fmt-check lint test ## Quicker check: verify format, lint, and test
	@echo "✅ Quicker checks passed!"

b: build ## Quick alias for build

r: run ## Quick alias for run

t: test ## Quick alias for test

c: clean ## Quick alias for clean

i: install ## Quick alias for install
