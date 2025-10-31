#!/bin/bash
set -e

echo "🔍 Starting MultiGit Verification..."
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

SUCCESS=0
FAILURES=0

run_test() {
    local test_name="$1"
    local test_command="$2"
    
    echo -n "Testing: $test_name... "
    
    if eval "$test_command" > /tmp/multigit_test.log 2>&1; then
        echo -e "${GREEN}✓ PASS${NC}"
        ((SUCCESS++))
        return 0
    else
        echo -e "${RED}✗ FAIL${NC}"
        echo -e "${YELLOW}Error details:${NC}"
        cat /tmp/multigit_test.log | head -5
        ((FAILURES++))
        return 1
    fi
}

echo -e "${BLUE}=== Compilation Checks ===${NC}"
run_test "Cargo check" "cargo check --all-targets"
run_test "Cargo build (release)" "cargo build --release"
echo ""

echo -e "${BLUE}=== Code Quality ===${NC}"
run_test "Clippy" "cargo clippy --all-targets --all-features -- -D warnings"
run_test "Format check" "cargo fmt -- --check"
echo ""

echo -e "${BLUE}=== Test Suite ===${NC}"
run_test "Unit tests" "cargo test --lib --quiet"
run_test "Integration tests" "cargo test --test '*' --quiet"
echo ""

echo -e "${BLUE}=== Security Checks ===${NC}"
if command -v cargo-audit &> /dev/null; then
    run_test "Cargo audit" "cargo audit"
else
    echo -e "${YELLOW}⚠ Skipping: cargo-audit not installed${NC}"
fi

run_test "No hardcoded secrets" "! git grep -E '(password|token|secret)\s*=\s*['\"]' | grep -v test | grep -v example | grep -v 'TOML' | grep -v '.md'"
echo ""

echo -e "${BLUE}=== Binary Verification ===${NC}"
run_test "Binary exists" "test -f target/release/multigit || test -f target/debug/multigit"

if [ -f target/release/multigit ]; then
    BINARY="target/release/multigit"
elif [ -f target/debug/multigit ]; then
    BINARY="target/debug/multigit"
else
    BINARY="cargo run --quiet --"
fi

run_test "Version command" "$BINARY --version"
run_test "Help command" "$BINARY --help"
echo ""

echo -e "${BLUE}=== Summary ===${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}✓ Passed: $SUCCESS${NC}"
if [ $FAILURES -gt 0 ]; then
    echo -e "${RED}✗ Failed: $FAILURES${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${RED}❌ Verification failed. Please check errors above.${NC}"
    exit 1
else
    echo -e "${GREEN}✗ Failed: 0${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo -e "${GREEN}╔════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  🎉 All checks passed!            ║${NC}"
    echo -e "${GREEN}║  ✅ MultiGit is SAFE & READY      ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════╝${NC}"
    echo ""
fi

# Cleanup
rm -f /tmp/multigit_test.log
