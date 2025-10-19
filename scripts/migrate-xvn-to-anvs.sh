#!/usr/bin/env bash
#
# migrate-xvn-to-anvs.sh
# Automated migration script from xvn to anvs
#
# Usage: ./scripts/migrate-xvn-to-anvs.sh
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== XVN to ANVS Migration Script ===${NC}"
echo
echo "This script will:"
echo "  1. Backup your xvn configuration"
echo "  2. Uninstall xvn"
echo "  3. Install anvs"
echo "  4. Help migrate your settings"
echo
read -p "Continue? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Migration cancelled."
    exit 0
fi

echo

# Step 1: Backup configuration
echo -e "${BLUE}Step 1: Backing up configuration${NC}"
if [ -f ~/.xvnrc ]; then
    cp ~/.xvnrc ~/.xvnrc.backup
    echo -e "${GREEN}✓${NC} Backed up ~/.xvnrc to ~/.xvnrc.backup"

    # Show current config
    echo
    echo "Your current configuration:"
    echo "---"
    cat ~/.xvnrc
    echo "---"
    echo
else
    echo -e "${YELLOW}ℹ${NC} No ~/.xvnrc found (using defaults)"
fi

# Step 2: Uninstall xvn
echo -e "${BLUE}Step 2: Uninstalling xvn${NC}"

# Try npm uninstall
if npm list -g @olvrcc/xvn &>/dev/null; then
    npm uninstall -g @olvrcc/xvn
    echo -e "${GREEN}✓${NC} Uninstalled @olvrcc/xvn from npm"
else
    echo -e "${YELLOW}ℹ${NC} xvn not installed via npm (skipping)"
fi

# Try Homebrew uninstall
if command -v brew &>/dev/null && brew list xvn &>/dev/null; then
    brew uninstall xvn
    echo -e "${GREEN}✓${NC} Uninstalled xvn from Homebrew"
fi

# Remove installation directory
if [ -d ~/.xvn ]; then
    rm -rf ~/.xvn
    echo -e "${GREEN}✓${NC} Removed ~/.xvn directory"
fi

# Step 3: Remove shell integration
echo
echo -e "${BLUE}Step 3: Removing shell integration${NC}"

PROFILES=(~/.bashrc ~/.zshrc ~/.bash_profile)
REMOVED=false

for profile in "${PROFILES[@]}"; do
    if [ -f "$profile" ]; then
        if grep -q "\.xvn/bin/xvn\.sh" "$profile"; then
            # Create backup
            cp "$profile" "$profile.xvn-migration-backup"

            # Remove line
            sed -i.tmp '/\.xvn\/bin\/xvn\.sh/d' "$profile"
            rm -f "$profile.tmp"

            echo -e "${GREEN}✓${NC} Removed xvn integration from $profile"
            REMOVED=true
        fi
    fi
done

if [ "$REMOVED" = false ]; then
    echo -e "${YELLOW}ℹ${NC} No shell integration found in profile files"
fi

# Step 4: Install anvs
echo
echo -e "${BLUE}Step 4: Installing anvs${NC}"

read -p "Install via npm or Homebrew? (npm/brew) [npm]: " INSTALL_METHOD
INSTALL_METHOD=${INSTALL_METHOD:-npm}

if [ "$INSTALL_METHOD" = "brew" ]; then
    if ! command -v brew &>/dev/null; then
        echo -e "${RED}✗${NC} Homebrew not found. Please install Homebrew or use npm."
        exit 1
    fi
    brew install olvrcc/anvs/anvs
elif [ "$INSTALL_METHOD" = "npm" ]; then
    npm install -g anvs
else
    echo -e "${RED}✗${NC} Invalid choice. Please run script again."
    exit 1
fi

# Verify installation
if command -v anvs &>/dev/null; then
    VERSION=$(anvs --version)
    echo -e "${GREEN}✓${NC} anvs installed successfully (version: $VERSION)"
else
    echo -e "${RED}✗${NC} Installation failed. Please check errors above."
    exit 1
fi

# Step 5: Run setup
echo
echo -e "${BLUE}Step 5: Running anvs setup${NC}"
anvs setup

# Step 6: Migrate configuration
echo
echo -e "${BLUE}Step 6: Migrating configuration${NC}"

if [ -f ~/.xvnrc.backup ]; then
    echo "Your old configuration has been backed up to ~/.xvnrc.backup"
    echo
    read -p "Copy settings to new config? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cp ~/.xvnrc.backup ~/.anvsrc
        echo -e "${GREEN}✓${NC} Configuration copied to ~/.anvsrc"
        echo
        echo "New configuration:"
        echo "---"
        cat ~/.anvsrc
        echo "---"
    else
        echo -e "${YELLOW}ℹ${NC} Skipped config migration. Edit ~/.anvsrc manually if needed."
    fi
fi

# Step 7: Project config files
echo
echo -e "${BLUE}Step 7: Project configuration files${NC}"
echo "You may have .xvn.yaml files in your projects."
echo "These need to be renamed to .anvs.yaml"
echo
read -p "Search for .xvn.yaml files in ~/projects? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if [ -d ~/projects ]; then
        echo "Searching for .xvn.yaml files..."
        FILES=$(find ~/projects -name ".xvn.yaml" -type f 2>/dev/null || true)

        if [ -n "$FILES" ]; then
            echo "Found:"
            echo "$FILES"
            echo
            read -p "Rename all to .anvs.yaml? (y/N) " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                find ~/projects -name ".xvn.yaml" -type f -execdir mv {} .anvs.yaml \; 2>/dev/null
                echo -e "${GREEN}✓${NC} Renamed all .xvn.yaml files to .anvs.yaml"
            else
                echo -e "${YELLOW}ℹ${NC} Skipped renaming. You can rename manually with:"
                echo "   mv .xvn.yaml .anvs.yaml"
            fi
        else
            echo -e "${YELLOW}ℹ${NC} No .xvn.yaml files found"
        fi
    else
        echo -e "${YELLOW}ℹ${NC} ~/projects directory not found"
    fi
fi

# Step 8: Final instructions
echo
echo -e "${GREEN}=== Migration Complete! ===${NC}"
echo
echo "Next steps:"
echo "  1. ${YELLOW}Restart your shell:${NC}"
echo "     exec \$SHELL -l"
echo
echo "  2. ${YELLOW}Verify installation:${NC}"
echo "     anvs --version"
echo "     anvs status"
echo
echo "  3. ${YELLOW}Test in a project:${NC}"
echo "     cd /path/to/project-with-nvmrc"
echo "     # Should automatically switch Node version"
echo
echo "Troubleshooting:"
echo "  - Documentation: https://github.com/olvrcc/anvs"
echo "  - Migration guide: docs/XVN_TO_ANVS_MIGRATION.md"
echo "  - Issues: https://github.com/olvrcc/anvs/issues"
echo
echo "Backups created:"
echo "  - Configuration: ~/.xvnrc.backup"
echo "  - Shell profiles: ~/.bashrc.xvn-migration-backup (if modified)"
echo
