#!/bin/bash
# MultiGit Multi-Remote Test Script
# This script tests pushing to both GitHub and GitLab

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

MULTIGIT="./target/release/multigit"

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}   MultiGit Multi-Remote Test${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Check if binary exists
if [ ! -f "$MULTIGIT" ]; then
    echo -e "${RED}❌ Binary not found. Building...${NC}"
    cargo build --release
fi

echo -e "${GREEN}Step 1: Current Status${NC}"
$MULTIGIT status
echo ""

echo -e "${GREEN}Step 2: Add GitHub Remote${NC}"
echo -e "${YELLOW}You need a GitHub token with 'repo' and 'read:user' scopes${NC}"
echo -e "${YELLOW}Get it from: https://github.com/settings/tokens${NC}"
echo ""

# Check if token is in environment
if [ -n "$MULTIGIT_GITHUB_TOKEN" ]; then
    echo -e "${GREEN}✓ Using token from MULTIGIT_GITHUB_TOKEN environment variable${NC}"
    export GITHUB_TOKEN="$MULTIGIT_GITHUB_TOKEN"
else
    echo -e "${YELLOW}⚠ No MULTIGIT_GITHUB_TOKEN found. Will prompt interactively.${NC}"
fi

# Add GitHub remote
if $MULTIGIT remote add github TIVerse; then
    echo -e "${GREEN}✓ GitHub remote added successfully${NC}"
else
    echo -e "${RED}❌ Failed to add GitHub remote${NC}"
    exit 1
fi
echo ""

echo -e "${GREEN}Step 3: Test GitHub Connection${NC}"
if $MULTIGIT remote test github; then
    echo -e "${GREEN}✓ GitHub connection successful${NC}"
else
    echo -e "${RED}❌ GitHub connection failed${NC}"
    exit 1
fi
echo ""

echo -e "${GREEN}Step 4: Add GitLab Remote${NC}"
echo -e "${YELLOW}You need a GitLab token with 'api' and 'write_repository' scopes${NC}"
echo -e "${YELLOW}Get it from: https://gitlab.com/-/profile/personal_access_tokens${NC}"
echo ""

# Check if token is in environment
if [ -n "$MULTIGIT_GITLAB_TOKEN" ]; then
    echo -e "${GREEN}✓ Using token from MULTIGIT_GITLAB_TOKEN environment variable${NC}"
    export GITLAB_TOKEN="$MULTIGIT_GITLAB_TOKEN"
else
    echo -e "${YELLOW}⚠ No MULTIGIT_GITLAB_TOKEN found. Will prompt interactively.${NC}"
fi

# Add GitLab remote
if $MULTIGIT remote add gitlab TIVisionOSS; then
    echo -e "${GREEN}✓ GitLab remote added successfully${NC}"
else
    echo -e "${RED}❌ Failed to add GitLab remote${NC}"
    exit 1
fi
echo ""

echo -e "${GREEN}Step 5: Test GitLab Connection${NC}"
if $MULTIGIT remote test gitlab; then
    echo -e "${GREEN}✓ GitLab connection successful${NC}"
else
    echo -e "${RED}❌ GitLab connection failed${NC}"
    exit 1
fi
echo ""

echo -e "${GREEN}Step 6: List All Remotes${NC}"
$MULTIGIT remote list
echo ""

echo -e "${GREEN}Step 7: Check Status${NC}"
$MULTIGIT status
echo ""

echo -e "${YELLOW}Step 8: DRY RUN - Test Sync (Safe, no actual push)${NC}"
read -p "Press Enter to run dry-run sync..."
$MULTIGIT sync --dry-run
echo ""

echo -e "${RED}Step 9: REAL PUSH - Push to Both Remotes${NC}"
echo -e "${YELLOW}This will actually push to GitHub AND GitLab!${NC}"
read -p "Are you sure you want to proceed? (yes/no): " confirm

if [ "$confirm" = "yes" ]; then
    echo -e "${GREEN}Pushing to both remotes...${NC}"
    if $MULTIGIT push; then
        echo ""
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${GREEN}   🎉 SUCCESS! Pushed to both remotes!${NC}"
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    else
        echo -e "${RED}❌ Push failed${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}Cancelled. No push performed.${NC}"
fi

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}   Test Complete!${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
