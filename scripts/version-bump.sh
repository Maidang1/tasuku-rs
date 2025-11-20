#!/bin/bash

# Script to process changesets and bump version
# This script:
# 1. Reads all changeset files
# 2. Determines the highest version bump needed
# 3. Updates Cargo.toml with the new version
# 4. Generates CHANGELOG.md entries
# 5. Removes processed changeset files

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

CHANGESET_DIR=".changeset"
CARGO_TOML="Cargo.toml"
CHANGELOG="CHANGELOG.md"

echo -e "${BLUE}=== Processing Changesets ===${NC}\n"

# Check if there are any changeset files
CHANGESET_FILES=$(find "$CHANGESET_DIR" -name "*.md" -not -name "README.md" 2>/dev/null || true)

if [ -z "$CHANGESET_FILES" ]; then
  echo -e "${YELLOW}No changesets found. Nothing to do.${NC}"
  exit 0
fi

echo -e "${GREEN}Found changesets:${NC}"
echo "$CHANGESET_FILES" | while read -r file; do
  echo "  - $(basename "$file")"
done
echo ""

# Determine the highest bump type
BUMP_TYPE="patch"
HAS_MINOR=false
HAS_MAJOR=false

echo "$CHANGESET_FILES" | while read -r file; do
  if grep -q '"tasuku-rs": major' "$file"; then
    HAS_MAJOR=true
  elif grep -q '"tasuku-rs": minor' "$file"; then
    HAS_MINOR=true
  fi
done

if [ "$HAS_MAJOR" = true ]; then
  BUMP_TYPE="major"
elif [ "$HAS_MINOR" = true ]; then
  BUMP_TYPE="minor"
fi

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' "$CARGO_TOML" | head -1 | sed 's/version = "\(.*\)"/\1/')
echo -e "${BLUE}Current version:${NC} $CURRENT_VERSION"

# Calculate new version
IFS='.' read -r -a VERSION_PARTS <<< "$CURRENT_VERSION"
MAJOR="${VERSION_PARTS[0]}"
MINOR="${VERSION_PARTS[1]}"
PATCH="${VERSION_PARTS[2]}"

case $BUMP_TYPE in
  major)
    MAJOR=$((MAJOR + 1))
    MINOR=0
    PATCH=0
    ;;
  minor)
    MINOR=$((MINOR + 1))
    PATCH=0
    ;;
  patch)
    PATCH=$((PATCH + 1))
    ;;
esac

NEW_VERSION="${MAJOR}.${MINOR}.${PATCH}"
echo -e "${BLUE}New version:${NC} $NEW_VERSION (${BUMP_TYPE} bump)"
echo ""

# Collect changelog entries
CHANGELOG_ENTRIES=""
echo "$CHANGESET_FILES" | while read -r file; do
  # Extract the description (everything after the frontmatter)
  DESCRIPTION=$(sed -n '/^---$/,/^---$/!p' "$file" | sed '/^$/d')
  if [ -n "$DESCRIPTION" ]; then
    echo "- $DESCRIPTION" >> /tmp/changeset_entries.tmp
  fi
done

if [ -f /tmp/changeset_entries.tmp ]; then
  CHANGELOG_ENTRIES=$(cat /tmp/changeset_entries.tmp)
  rm /tmp/changeset_entries.tmp
fi

# Update Cargo.toml
echo -e "${YELLOW}Updating Cargo.toml...${NC}"
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"
rm "${CARGO_TOML}.bak"
echo -e "${GREEN}✓ Updated Cargo.toml${NC}\n"

# Update CHANGELOG.md
echo -e "${YELLOW}Updating CHANGELOG.md...${NC}"
CURRENT_DATE=$(date +%Y-%m-%d)

# Create new changelog entry
NEW_ENTRY="## [$NEW_VERSION] - $CURRENT_DATE

$CHANGELOG_ENTRIES

"

# Check if CHANGELOG.md exists
if [ ! -f "$CHANGELOG" ]; then
  # Create new CHANGELOG.md
  cat > "$CHANGELOG" << EOF
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

$NEW_ENTRY
EOF
else
  # Insert new entry after the header
  # Find the line number after the header (after "## " or at the beginning of version entries)
  LINE_NUM=$(grep -n "^## \[" "$CHANGELOG" | head -1 | cut -d: -f1)
  
  if [ -z "$LINE_NUM" ]; then
    # No existing version entries, append to end
    echo "$NEW_ENTRY" >> "$CHANGELOG"
  else
    # Insert before the first version entry
    {
      head -n $((LINE_NUM - 1)) "$CHANGELOG"
      echo "$NEW_ENTRY"
      tail -n +$LINE_NUM "$CHANGELOG"
    } > "${CHANGELOG}.tmp"
    mv "${CHANGELOG}.tmp" "$CHANGELOG"
  fi
fi

echo -e "${GREEN}✓ Updated CHANGELOG.md${NC}\n"

# Remove processed changeset files
echo -e "${YELLOW}Removing processed changesets...${NC}"
echo "$CHANGESET_FILES" | while read -r file; do
  rm "$file"
  echo "  - Removed $(basename "$file")"
done
echo -e "${GREEN}✓ Changesets processed${NC}\n"

echo -e "${GREEN}=== Version Bump Complete ===${NC}"
echo -e "${BLUE}Version:${NC} $CURRENT_VERSION → $NEW_VERSION"
echo -e "\n${YELLOW}Next steps:${NC}"
echo "  1. Review the changes in Cargo.toml and CHANGELOG.md"
echo "  2. Commit the changes:"
echo "     ${BLUE}git add Cargo.toml CHANGELOG.md .changeset${NC}"
echo "     ${BLUE}git commit -m \"chore: release v$NEW_VERSION\"${NC}"
echo "  3. Create and push a tag:"
echo "     ${BLUE}git tag v$NEW_VERSION${NC}"
echo "     ${BLUE}git push origin main --tags${NC}"
