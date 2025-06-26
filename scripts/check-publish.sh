#!/bin/bash
set -e

echo "🔍 Pre-publish checks for zahuyach"
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get package info
PACKAGE_NAME=$(grep '^name' Cargo.toml | sed 's/name = "\(.*\)"/\1/')
PACKAGE_VERSION=$(grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

echo -e "\n📦 Package: $PACKAGE_NAME v$PACKAGE_VERSION"

# Check 1: Verify Cargo.toml has required fields
echo -e "\n1️⃣  Checking Cargo.toml requirements..."
REQUIRED_FIELDS=("name" "version" "authors" "edition" "description" "license" "repository")
MISSING_FIELDS=()

for field in "${REQUIRED_FIELDS[@]}"; do
    if ! grep -q "^$field" Cargo.toml; then
        MISSING_FIELDS+=("$field")
    fi
done

if [ ${#MISSING_FIELDS[@]} -eq 0 ]; then
    echo -e "${GREEN}✓ All required fields present${NC}"
else
    echo -e "${RED}✗ Missing fields: ${MISSING_FIELDS[*]}${NC}"
    exit 1
fi

# Check 2: Format check
echo -e "\n2️⃣  Checking code formatting..."
if cargo fmt -- --check > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Code is properly formatted${NC}"
else
    echo -e "${YELLOW}⚠ Code needs formatting. Run 'cargo fmt'${NC}"
fi

# Check 3: Run clippy
echo -e "\n3️⃣  Running clippy..."
if cargo clippy --all-targets --all-features -- -D warnings > /dev/null 2>&1; then
    echo -e "${GREEN}✓ No clippy warnings${NC}"
else
    echo -e "${YELLOW}⚠ Clippy found issues. Run 'cargo clippy'${NC}"
fi

# Check 4: Run tests
echo -e "\n4️⃣  Running tests..."
if cargo test --quiet; then
    echo -e "${GREEN}✓ All tests passed${NC}"
else
    echo -e "${RED}✗ Tests failed${NC}"
    exit 1
fi

# Check 5: Build release
echo -e "\n5️⃣  Building release..."
if cargo build --release --quiet; then
    echo -e "${GREEN}✓ Release build successful${NC}"
else
    echo -e "${RED}✗ Release build failed${NC}"
    exit 1
fi

# Check 6: Check if version exists on crates.io
echo -e "\n6️⃣  Checking crates.io..."
if curl -s "https://crates.io/api/v1/crates/$PACKAGE_NAME" | grep -q "\"name\":\"$PACKAGE_NAME\""; then
    echo -e "${GREEN}✓ Package exists on crates.io${NC}"

    # Check if this version is already published
    if curl -s "https://crates.io/api/v1/crates/$PACKAGE_NAME/$PACKAGE_VERSION" | grep -q "\"version\":\"$PACKAGE_VERSION\""; then
        echo -e "${YELLOW}⚠ Version $PACKAGE_VERSION is already published!${NC}"
        echo -e "  You need to bump the version before publishing."
    else
        echo -e "${GREEN}✓ Version $PACKAGE_VERSION is not yet published${NC}"
    fi
else
    echo -e "${GREEN}✓ Package not yet on crates.io (first release)${NC}"
fi

# Check 7: Dry run
echo -e "\n7️⃣  Performing dry run..."
if cargo publish --dry-run > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Dry run successful${NC}"
else
    echo -e "${RED}✗ Dry run failed. Check 'cargo publish --dry-run' output${NC}"
    exit 1
fi

# Check 8: Git status
echo -e "\n8️⃣  Checking git status..."
if [[ -z $(git status -s) ]]; then
    echo -e "${GREEN}✓ Working directory clean${NC}"
else
    echo -e "${YELLOW}⚠ Uncommitted changes detected${NC}"
    git status -s
fi

# Summary
echo -e "\n✅ ${GREEN}All checks passed!${NC}"
echo -e "\nTo publish, run:"
echo -e "  ${YELLOW}cargo publish${NC}"
echo -e "\nOr push to main branch for automatic release."
