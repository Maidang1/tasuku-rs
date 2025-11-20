#!/bin/bash

# Script to interactively create a changeset file
# Usage: ./scripts/add-changeset.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Create a Changeset ===${NC}\n"

# Prompt for change type
echo -e "${YELLOW}What type of change is this?${NC}"
echo "  1) patch   - Bug fixes and minor changes (0.1.0 → 0.1.1)"
echo "  2) minor   - New features (0.1.0 → 0.2.0)"
echo "  3) major   - Breaking changes (0.1.0 → 1.0.0)"
echo ""
read -p "Enter your choice (1-3): " choice

case $choice in
  1)
    BUMP_TYPE="patch"
    ;;
  2)
    BUMP_TYPE="minor"
    ;;
  3)
    BUMP_TYPE="major"
    ;;
  *)
    echo -e "${RED}Invalid choice. Exiting.${NC}"
    exit 1
    ;;
esac

echo -e "\n${YELLOW}Describe your change:${NC}"
echo "(This will appear in the CHANGELOG)"
read -p "> " DESCRIPTION

if [ -z "$DESCRIPTION" ]; then
  echo -e "${RED}Description cannot be empty. Exiting.${NC}"
  exit 1
fi

# Generate a unique filename based on timestamp and random string
TIMESTAMP=$(date +%s)
RANDOM_STR=$(cat /dev/urandom | LC_ALL=C tr -dc 'a-z0-9' | fold -w 8 | head -n 1)
FILENAME="${BUMP_TYPE}-${TIMESTAMP}-${RANDOM_STR}.md"
FILEPATH=".changeset/${FILENAME}"

# Create the changeset file
cat > "$FILEPATH" << EOF
---
"tasuku-rs": ${BUMP_TYPE}
---

${DESCRIPTION}
EOF

echo -e "\n${GREEN}✓ Changeset created successfully!${NC}"
echo -e "  File: ${BLUE}${FILEPATH}${NC}"
echo -e "  Type: ${BLUE}${BUMP_TYPE}${NC}"
echo -e "\n${YELLOW}Next steps:${NC}"
echo "  1. Review the changeset file"
echo "  2. Commit it along with your code changes"
echo "  3. Open a pull request"
