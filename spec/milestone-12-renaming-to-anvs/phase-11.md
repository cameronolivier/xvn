# Phase 11: Migration Guide

**Status**: Not Started
**Version**: v2.0.0
**Duration Estimate**: 1-2 hours

## Overview

Phase 11 creates comprehensive documentation to help existing `xvn` users migrate to `anvs`. This phase is critical for user success and adoption of the renamed package. The migration guide must be clear, thorough, and account for various user scenarios and potential issues.

**Why Phase 11 is Essential:**
- Provides step-by-step instructions for existing users to migrate
- Reduces support burden by proactively addressing common issues
- Builds user confidence in the migration process
- Documents configuration migration and compatibility considerations
- Creates a reference for troubleshooting migration problems

**⚠️ CHECKPOINT**: This phase should be completed before publishing v2.0.0 to ensure users have clear migration instructions from day one.

---

## Implementation Tasks

### Task 11.1: Create Migration Documentation

**File**: `docs/XVN_TO_ANVS_MIGRATION.md` (new file)

**Content Requirements**:

```markdown
# Migrating from XVN to ANVS

This guide provides step-by-step instructions for migrating from `xvn` to `anvs`.

## Overview

The `xvn` project has been renamed to `anvs` (Automatic Node Version Switcher) with the following changes:

| Component           | Old (xvn)                | New (anvs)               |
|---------------------|--------------------------|--------------------------|
| npm Package         | `@olvrcc/xvn`            | `anvs`                   |
| Binary Name         | `xvn`                    | `anvs`                   |
| Install Directory   | `~/.xvn/`                | `~/.anvs/`               |
| Global Config       | `~/.xvnrc`               | `~/.anvsrc`              |
| Project Config      | `.xvn.yaml`              | `.anvs.yaml`             |
| Shell Script        | `shell/xvn.sh`           | `shell/anvs.sh`          |
| Environment Vars    | `XVN_*`                  | `ANVS_*`                 |
| GitHub Repository   | `olvrcc/xvn`             | `olvrcc/anvs`            |
| Homebrew Tap        | `olvrcc/homebrew-xvn`    | `olvrcc/homebrew-anvs`   |

**Version**: `anvs` starts at version `2.0.0`

---

## Prerequisites

Before starting the migration:

1. **Backup your configuration** (recommended):
   ```bash
   # Backup global config
   cp ~/.xvnrc ~/.xvnrc.backup 2>/dev/null || echo "No global config found"

   # Backup project configs (run in your projects)
   find ~/projects -name ".xvn.yaml" -exec cp {} {}.backup \;
   ```

2. **Note your current settings**:
   ```bash
   # View your current config
   cat ~/.xvnrc

   # Check which version managers you're using
   xvn status
   ```

3. **Ensure you have npm or Homebrew** for installation

---

## Migration Methods

Choose the method that best fits your installation:

- [Quick Migration](#quick-migration) - Fast automated migration (recommended)
- [Manual Migration](#manual-migration) - Step-by-step manual process
- [Clean Install](#clean-install) - Fresh start without config migration

---

## Quick Migration

**Estimated time**: 2-5 minutes

This method uninstalls `xvn`, installs `anvs`, and helps migrate your configuration.

### Step 1: Uninstall XVN

```bash
# Option A: Using the uninstall command (recommended)
xvn uninstall

# Option B: Manual npm uninstall
npm uninstall -g @olvrcc/xvn
rm -rf ~/.xvn

# Option C: If installed via Homebrew
brew uninstall xvn
rm -rf ~/.xvn
```

**Verify uninstallation**:
```bash
which xvn  # Should show: xvn not found
ls ~/.xvn  # Should show: No such file or directory
```

### Step 2: Remove Shell Integration

Open your shell profile and remove the `xvn` source line:

**For bash** (`~/.bashrc` or `~/.bash_profile`):
```bash
# Remove or comment out this line:
# [ -s "$HOME/.xvn/bin/xvn.sh" ] && . "$HOME/.xvn/bin/xvn.sh"
```

**For zsh** (`~/.zshrc`):
```bash
# Remove or comment out this line:
# [ -s "$HOME/.xvn/bin/xvn.sh" ] && . "$HOME/.xvn/bin/xvn.sh"
```

**Quick removal** (automated):
```bash
# For bash
sed -i.bak '/\.xvn\/bin\/xvn\.sh/d' ~/.bashrc

# For zsh
sed -i.bak '/\.xvn\/bin\/xvn\.sh/d' ~/.zshrc
```

### Step 3: Install ANVS

**Option A: via npm (recommended)**
```bash
npm install -g anvs
```

**Option B: via Homebrew**
```bash
brew install olvrcc/anvs/anvs
```

**Verify installation**:
```bash
which anvs    # Should show path to anvs
anvs --version  # Should show: 2.0.0 or higher
```

### Step 4: Run Setup

```bash
anvs setup
```

This will:
- Create `~/.anvs/` directory
- Install the `anvs` binary
- Add shell integration to your profile
- Create default `~/.anvsrc` if it doesn't exist

**Expected output**:
```
✓ anvs binary installed to ~/.anvs/bin/anvs
✓ Shell integration added to ~/.zshrc
✓ Configuration file created at ~/.anvsrc
✓ Setup complete! Restart your shell or run: source ~/.zshrc
```

### Step 5: Migrate Configuration

If you had custom settings in `~/.xvnrc`, migrate them manually:

```bash
# View old config (from backup)
cat ~/.xvnrc.backup

# Edit new config
nano ~/.anvsrc  # or vim, code, etc.
```

**Common settings to migrate**:
- `version_managers`: List of enabled version managers
- `auto_install`: Whether to prompt for missing versions
- `debug`: Debug logging setting

**Example migration**:

Old `~/.xvnrc`:
```yaml
version_managers:
  - nvm
  - fnm
auto_install: true
debug: false
```

New `~/.anvsrc`:
```yaml
version_managers:
  - nvm
  - fnm
auto_install: true
debug: false
```

### Step 6: Migrate Project Configurations

For each project with a `.xvn.yaml` file, rename it to `.anvs.yaml`:

```bash
# In each project directory:
mv .xvn.yaml .anvs.yaml
```

**Bulk migration** (finds and renames all `.xvn.yaml` files):
```bash
# Dry run (shows what would be renamed)
find ~/projects -name ".xvn.yaml" -type f

# Actual rename
find ~/projects -name ".xvn.yaml" -type f -execdir mv {} .anvs.yaml \;
```

### Step 7: Restart Shell

```bash
# For bash
source ~/.bashrc

# For zsh
source ~/.zshrc

# Or just restart your terminal
```

### Step 8: Verify Migration

```bash
# Check version
anvs --version  # Should show: 2.0.0 or higher

# Check configuration
anvs status

# Test activation in a project with .nvmrc
cd /path/to/project-with-nvmrc
# Should automatically switch Node.js version
```

---

## Manual Migration

**Estimated time**: 5-10 minutes

Prefer to do each step manually? Follow this detailed process:

### 1. Note Current Configuration

```bash
# Save current config
cp ~/.xvnrc ~/xvn-migration-backup.yaml 2>/dev/null || echo "No config"

# Note which version managers you use
xvn status
```

### 2. Uninstall XVN Completely

```bash
# Remove shell integration from profile
# Edit ~/.bashrc or ~/.zshrc and remove the xvn source line

# Uninstall package
npm uninstall -g @olvrcc/xvn

# Remove installation directory
rm -rf ~/.xvn

# Remove old config (we have a backup)
rm -f ~/.xvnrc
```

### 3. Clean Shell Environment

```bash
# Restart shell to clear old environment
exec $SHELL -l

# Verify xvn is gone
which xvn  # Should show: not found
```

### 4. Install ANVS

```bash
# Via npm
npm install -g anvs

# Verify
which anvs
anvs --version
```

### 5. Configure ANVS

```bash
# Run setup
anvs setup

# Restore your settings manually
nano ~/.anvsrc
# Copy settings from ~/xvn-migration-backup.yaml
```

### 6. Update Project Files

```bash
# In each project, rename config file
mv .xvn.yaml .anvs.yaml
```

### 7. Reload Shell

```bash
source ~/.zshrc  # or ~/.bashrc
```

### 8. Test

```bash
anvs status
cd /path/to/project
# Verify automatic version switching works
```

---

## Clean Install

Want to start fresh? This method installs `anvs` without migrating old settings:

### 1. Remove XVN (if installed)

```bash
npm uninstall -g @olvrcc/xvn
rm -rf ~/.xvn ~/.xvnrc
# Remove shell integration from profile manually
```

### 2. Install ANVS

```bash
npm install -g anvs
anvs setup
```

### 3. Configure from Scratch

```bash
# Edit config
nano ~/.anvsrc

# Set up project configs
cd /path/to/project
echo "16.20.0" > .nvmrc  # or create .anvs.yaml
```

---

## Troubleshooting

### Issue: "anvs: command not found" after installation

**Causes**:
- npm global bin directory not in PATH
- Shell profile not reloaded
- Installation failed

**Solutions**:
```bash
# Check if anvs is installed
npm list -g anvs

# Check npm global bin path
npm bin -g

# Ensure PATH includes npm global bin
echo $PATH | grep "$(npm bin -g)"

# If not in PATH, add to ~/.zshrc or ~/.bashrc:
export PATH="$(npm bin -g):$PATH"

# Reload shell
source ~/.zshrc  # or ~/.bashrc
```

### Issue: Shell integration not working

**Symptoms**: Version doesn't switch automatically on `cd`

**Solutions**:
```bash
# Verify setup ran
grep -r "anvs.sh" ~/.zshrc ~/.bashrc

# Expected to find:
# [ -s "$HOME/.anvs/bin/anvs.sh" ] && . "$HOME/.anvs/bin/anvs.sh"

# If not present, run setup again
anvs setup

# Reload shell
source ~/.zshrc
```

### Issue: "XVN_*" environment variables still present

**Symptoms**: Old `XVN_DEBUG`, `XVN_VERSION_FILES`, etc. still in environment

**Solutions**:
```bash
# These are set by the old shell script
# Remove old shell integration and restart shell

# Edit profile and remove xvn.sh source line
nano ~/.zshrc

# Restart shell completely
exec $SHELL -l

# Verify old vars are gone
env | grep XVN  # Should show nothing
env | grep ANVS  # Should show ANVS_* variables
```

### Issue: Both xvn and anvs are installed

**Symptoms**: Both commands exist

**Solutions**:
```bash
# Check installation locations
which xvn
which anvs

# Uninstall xvn completely
npm uninstall -g @olvrcc/xvn
brew uninstall xvn 2>/dev/null

# Remove old installation
rm -rf ~/.xvn

# Verify only anvs remains
which xvn  # Should show: not found
which anvs  # Should show anvs path
```

### Issue: Old .xvn.yaml files not recognized

**Symptoms**: anvs doesn't activate in projects with `.xvn.yaml`

**Solution**:
```bash
# anvs only recognizes .anvs.yaml
# Rename project config files
mv .xvn.yaml .anvs.yaml

# Or use standard version files
echo "16.20.0" > .nvmrc
```

### Issue: Configuration not loading

**Symptoms**: `anvs status` shows defaults, not custom config

**Solutions**:
```bash
# Check config file location
ls -la ~/.anvsrc

# Verify config syntax
cat ~/.anvsrc

# Config must be valid YAML
# Common issues:
# - Tabs instead of spaces (use 2 spaces)
# - Incorrect indentation
# - Missing quotes around strings

# Test with minimal config
echo "version_managers:" > ~/.anvsrc
echo "  - nvm" >> ~/.anvsrc
anvs status
```

### Issue: Permission denied errors

**Symptoms**: Errors like "Permission denied" during installation

**Solutions**:
```bash
# Don't use sudo with npm global install
# Fix npm permissions:
mkdir -p ~/.npm-global
npm config set prefix '~/.npm-global'
echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.zshrc
source ~/.zshrc

# Then install
npm install -g anvs
```

### Issue: Version not switching automatically

**Symptoms**: `cd` into project doesn't activate version

**Debug checklist**:
```bash
# 1. Check shell integration loaded
echo $ANVS_SHELL_LOADED  # Should show: 1

# 2. Check version file exists
ls -la .nvmrc .node-version package.json .anvs.yaml

# 3. Check version manager installed
which nvm
# or
which fnm

# 4. Enable debug mode
export ANVS_DEBUG=1
cd .  # Re-trigger activation
# Should show debug output

# 5. Test manual activation
anvs activate
```

---

## Migration Script (Optional)

For advanced users, here's a complete migration script that automates the entire process:

**File**: `scripts/migrate-xvn-to-anvs.sh`

```bash
#!/usr/bin/env bash
set -e

echo "=== XVN to ANVS Migration Script ==="
echo

# Backup config
if [ -f ~/.xvnrc ]; then
    echo "✓ Backing up ~/.xvnrc to ~/.xvnrc.backup"
    cp ~/.xvnrc ~/.xvnrc.backup
else
    echo "ℹ No ~/.xvnrc found (skipping backup)"
fi

# Uninstall xvn
echo
echo "Uninstalling xvn..."
npm uninstall -g @olvrcc/xvn 2>/dev/null || echo "  (not installed via npm)"

# Remove directory
if [ -d ~/.xvn ]; then
    echo "✓ Removing ~/.xvn directory"
    rm -rf ~/.xvn
fi

# Remove shell integration
echo
echo "Removing shell integration..."
for profile in ~/.bashrc ~/.zshrc ~/.bash_profile; do
    if [ -f "$profile" ]; then
        if grep -q "\.xvn/bin/xvn\.sh" "$profile"; then
            echo "✓ Removing from $profile"
            sed -i.bak '/\.xvn\/bin\/xvn\.sh/d' "$profile"
        fi
    fi
done

# Install anvs
echo
echo "Installing anvs..."
npm install -g anvs

# Run setup
echo
echo "Running anvs setup..."
anvs setup

# Migrate config
if [ -f ~/.xvnrc.backup ]; then
    echo
    echo "Migration complete! Next steps:"
    echo
    echo "1. Review and migrate your config:"
    echo "   Old config: ~/.xvnrc.backup"
    echo "   New config: ~/.anvsrc"
    echo
    echo "2. Rename project config files:"
    echo "   find ~/projects -name '.xvn.yaml' -execdir mv {} .anvs.yaml \\;"
    echo
    echo "3. Restart your shell:"
    echo "   exec \$SHELL -l"
    echo
else
    echo
    echo "Migration complete!"
    echo "Restart your shell: exec \$SHELL -l"
fi

echo
echo "✓ Migration script finished"
```

**Usage**:
```bash
# Download and run
curl -fsSL https://raw.githubusercontent.com/olvrcc/anvs/main/scripts/migrate-xvn-to-anvs.sh | bash

# Or run locally
chmod +x scripts/migrate-xvn-to-anvs.sh
./scripts/migrate-xvn-to-anvs.sh
```

---

## FAQ

### Why was xvn renamed to anvs?

- **Better package name**: `anvs` is unnamespaced on npm (vs `@olvrcc/xvn`)
- **Clearer purpose**: "Automatic Node Version Switcher" immediately explains what it does
- **Improved discoverability**: Easier to find and remember
- **Tribute to avn**: Honors the original project while being distinct

### Will my xvn installation stop working?

No! Your existing `xvn` installation will continue to work indefinitely. However:
- No new features will be added to `xvn`
- Bug fixes and updates only go to `anvs`
- We recommend migrating when convenient

### Can I keep both xvn and anvs installed?

Technically yes, but **not recommended** because:
- Both will try to hook into shell `cd` events
- Configuration and state will conflict
- Only one should manage version switching

Choose one and uninstall the other.

### Do I need to update my CI/CD pipelines?

If your CI/CD uses `xvn`, update to `anvs`:

**Before**:
```yaml
- npm install -g @olvrcc/xvn
- xvn activate
```

**After**:
```yaml
- npm install -g anvs
- anvs activate
```

### What about my existing .nvmrc files?

No changes needed! `anvs` reads the same version files as `xvn`:
- `.nvmrc`
- `.node-version`
- `package.json` (`engines.node`)
- `.anvs.yaml` (replaces `.xvn.yaml`)

### Can I migrate config automatically?

The config format is identical (YAML), so you can copy settings directly:

```bash
# If you have custom settings
cp ~/.xvnrc ~/.anvsrc

# Or just copy the content manually
```

### Is there a downgrade path?

Yes, you can go back to `xvn`:

```bash
# Uninstall anvs
npm uninstall -g anvs
rm -rf ~/.anvs

# Reinstall xvn
npm install -g @olvrcc/xvn
xvn setup

# Restore config
mv ~/.anvsrc.backup ~/.xvnrc
```

Note: `xvn` won't receive updates, so only downgrade if absolutely necessary.

### Where can I get help?

- GitHub Issues: https://github.com/olvrcc/anvs/issues
- Documentation: https://github.com/olvrcc/anvs
- Migration Guide: This document

---

## Checklist

Use this checklist to track your migration progress:

- [ ] Backup configuration: `cp ~/.xvnrc ~/.xvnrc.backup`
- [ ] Note current settings: `xvn status`
- [ ] Uninstall xvn: `xvn uninstall` or `npm uninstall -g @olvrcc/xvn`
- [ ] Remove shell integration from `~/.bashrc` or `~/.zshrc`
- [ ] Remove `~/.xvn` directory
- [ ] Install anvs: `npm install -g anvs`
- [ ] Verify installation: `anvs --version`
- [ ] Run setup: `anvs setup`
- [ ] Migrate config: Copy settings from `~/.xvnrc.backup` to `~/.anvsrc`
- [ ] Rename project configs: `mv .xvn.yaml .anvs.yaml`
- [ ] Restart shell: `source ~/.zshrc` or restart terminal
- [ ] Test status: `anvs status`
- [ ] Test activation: `cd` into project with `.nvmrc`
- [ ] Verify version switching works
- [ ] Clean up backups (optional): `rm ~/.xvnrc.backup`

---

## Additional Resources

- [Main README](../README.md) - Full documentation for anvs
- [Architecture Guide](./ARCHITECTURE.md) - How anvs works internally
- [Contributing Guide](../CONTRIBUTING.md) - How to contribute
- [Changelog](../CHANGELOG.md) - Version history

---

**Need help?** Open an issue at https://github.com/olvrcc/anvs/issues
```

**Actions**:
- [ ] Create `docs/XVN_TO_ANVS_MIGRATION.md` with above content
- [ ] Review for clarity and completeness
- [ ] Test all commands in a clean environment
- [ ] Verify links and references are correct

---

### Task 11.2: Create Migration Script (Optional)

**File**: `scripts/migrate-xvn-to-anvs.sh` (new file)

**Purpose**: Automated migration script to simplify the process for users.

**Content**:
```bash
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
```

**Actions**:
- [ ] Create `scripts/migrate-xvn-to-anvs.sh` with above content
- [ ] Make executable: `chmod +x scripts/migrate-xvn-to-anvs.sh`
- [ ] Test script in a clean environment
- [ ] Verify script handles edge cases (no xvn installed, etc.)
- [ ] Add error handling for common issues

---

### Task 11.3: Update Main Documentation with Migration Links

**File**: `README.md`

**Changes Required**:

Add a "Migration" section after the "Installation" section:

```markdown
## Migration from XVN

If you're upgrading from `xvn` to `anvs`, see the [Migration Guide](docs/XVN_TO_ANVS_MIGRATION.md).

**Quick migration**:
```bash
xvn uninstall
npm install -g anvs
anvs setup
```

For detailed instructions, troubleshooting, and configuration migration, see [docs/XVN_TO_ANVS_MIGRATION.md](docs/XVN_TO_ANVS_MIGRATION.md).
```

**Actions**:
- [ ] Add migration section to README.md
- [ ] Ensure link to migration guide is correct
- [ ] Place section prominently (after Installation)

---

### Task 11.4: Update MIGRATION.md with XVN Migration Section

**File**: `docs/MIGRATION.md`

**Changes Required**:

Add a new section at the top of the file for the xvn → anvs migration:

```markdown
## Migrating from XVN to ANVS (v2.0.0)

The project has been renamed from `xvn` to `anvs` in version 2.0.0. This is a **breaking change** that requires manual migration.

### Quick Migration Steps

1. Backup config: `cp ~/.xvnrc ~/.xvnrc.backup`
2. Uninstall xvn: `xvn uninstall`
3. Install anvs: `npm install -g anvs`
4. Run setup: `anvs setup`
5. Migrate config: Copy settings from `~/.xvnrc.backup` to `~/.anvsrc`
6. Rename project configs: `mv .xvn.yaml .anvs.yaml`

### Complete Migration Guide

See [XVN_TO_ANVS_MIGRATION.md](./XVN_TO_ANVS_MIGRATION.md) for:
- Detailed step-by-step instructions
- Troubleshooting common issues
- Automated migration script
- FAQ

---
```

**Actions**:
- [ ] Add xvn → anvs migration section to `docs/MIGRATION.md`
- [ ] Place at the top of the file (most relevant migration)
- [ ] Link to detailed migration guide

---

### Task 11.5: Update CHANGELOG with Migration Info

**File**: `CHANGELOG.md`

**Changes Required**:

Update the v2.0.0 entry to include migration information:

```markdown
## [2.0.0] - 2025-10-19

### ⚠️ BREAKING CHANGES

This release renames the entire project from `xvn` to `anvs` (Automatic Node Version Switcher).

**What changed:**
- Package name: `@olvrcc/xvn` → `anvs`
- Binary name: `xvn` → `anvs`
- Install path: `~/.xvn/` → `~/.anvs/`
- Config files: `~/.xvnrc` → `~/.anvsrc`, `.xvn.yaml` → `.anvs.yaml`
- Repository: `olvrcc/xvn` → `olvrcc/anvs`

**Migration required**: This is NOT an automatic upgrade. Existing `xvn` users must manually migrate.

**Migration guide**: See [docs/XVN_TO_ANVS_MIGRATION.md](docs/XVN_TO_ANVS_MIGRATION.md)

**Quick migration**:
```bash
xvn uninstall
npm install -g anvs
anvs setup
```

### Added
- Comprehensive migration documentation
- Automated migration script (`scripts/migrate-xvn-to-anvs.sh`)
- Migration troubleshooting guide

### Changed
- All references to `xvn` renamed to `anvs` throughout codebase
- Package published to npm as `anvs` (unnamespaced)
- Repository renamed on GitHub

### Deprecated
- `@olvrcc/xvn` package deprecated on npm (will continue to work)
- Final `xvn` version is v1.7.0

---
```

**Actions**:
- [ ] Update v2.0.0 CHANGELOG entry
- [ ] Add migration information prominently
- [ ] Link to migration guide
- [ ] Note deprecation of old package

---

### Task 11.6: Test Migration Documentation

**Test the migration process**:

```bash
# Set up a test environment
TEST_HOME=/tmp/migration-test
mkdir -p $TEST_HOME

# Create mock xvn installation
mkdir -p $TEST_HOME/.xvn/bin
echo "#!/bin/bash" > $TEST_HOME/.xvn/bin/xvn
echo 'echo "xvn 1.6.2"' >> $TEST_HOME/.xvn/bin/xvn
chmod +x $TEST_HOME/.xvn/bin/xvn

# Create mock config
cat > $TEST_HOME/.xvnrc << EOF
version_managers:
  - nvm
  - fnm
auto_install: true
debug: false
EOF

# Create mock shell profile
echo '[ -s "$HOME/.xvn/bin/xvn.sh" ] && . "$HOME/.xvn/bin/xvn.sh"' > $TEST_HOME/.zshrc

# Test migration script
HOME=$TEST_HOME ./scripts/migrate-xvn-to-anvs.sh

# Verify results
ls -la $TEST_HOME/.anvs
cat $TEST_HOME/.anvsrc
grep "anvs" $TEST_HOME/.zshrc
```

**Actions**:
- [ ] Test migration script in clean environment
- [ ] Verify all steps work as documented
- [ ] Test with various scenarios:
  - [ ] Fresh install (no xvn)
  - [ ] Existing xvn installation
  - [ ] Custom configuration
  - [ ] Multiple shell profiles
- [ ] Document any issues found
- [ ] Update docs based on test results

---

### Task 11.7: Create Migration Announcement Template

**File**: `docs/MIGRATION_ANNOUNCEMENT.md` (new file)

**Purpose**: Template for announcing the migration to users via GitHub issues, discussions, etc.

**Content**:
```markdown
# Announcement: XVN Renamed to ANVS

## 🎉 Introducing ANVS - Automatic Node Version Switcher

We're excited to announce that `xvn` has been renamed to **anvs** (Automatic Node Version Switcher)!

### Why the Change?

- **Better Package Name**: `anvs` is available as an unnamespaced npm package (vs `@olvrcc/xvn`)
- **Clearer Purpose**: The name immediately tells you what it does
- **Improved Discoverability**: Easier to find and remember
- **Tribute to AVN**: Honors the original `avn` project while being distinct

### What's New?

Version 2.0.0 brings:
- New package name: `anvs` on npm
- Same great features and performance
- Comprehensive migration guide
- Automated migration script

### For Existing Users

**Your current `xvn` installation will continue to work!** However:
- No new features will be added to `xvn`
- All future development happens on `anvs`
- We recommend migrating when convenient

### Quick Migration

```bash
xvn uninstall
npm install -g anvs
anvs setup
```

### Full Documentation

- **Migration Guide**: [docs/XVN_TO_ANVS_MIGRATION.md](docs/XVN_TO_ANVS_MIGRATION.md)
- **Automated Script**: [scripts/migrate-xvn-to-anvs.sh](scripts/migrate-xvn-to-anvs.sh)
- **Troubleshooting**: See migration guide

### Installation

**New users** can install directly:
```bash
npm install -g anvs
anvs setup
```

Or via Homebrew:
```bash
brew install olvrcc/anvs/anvs
```

### Need Help?

- 📖 [Migration Guide](docs/XVN_TO_ANVS_MIGRATION.md)
- 🐛 [Report Issues](https://github.com/olvrcc/anvs/issues)
- 💬 [Discussions](https://github.com/olvrcc/anvs/discussions)

Thank you for your continued support! 🙏
```

**Actions**:
- [ ] Create announcement template
- [ ] Customize for different channels (GitHub, npm, etc.)
- [ ] Review tone and messaging

---

## Verification Checklist

Before proceeding to Phase 12, verify ALL of the following:

- [ ] `docs/XVN_TO_ANVS_MIGRATION.md` created with comprehensive instructions
- [ ] Migration script `scripts/migrate-xvn-to-anvs.sh` created and tested
- [ ] Script is executable: `chmod +x scripts/migrate-xvn-to-anvs.sh`
- [ ] README.md includes migration section with link
- [ ] `docs/MIGRATION.md` updated with xvn → anvs section
- [ ] CHANGELOG.md v2.0.0 entry includes migration info
- [ ] All commands in migration guide tested and verified
- [ ] Migration script tested in clean environment
- [ ] Troubleshooting section addresses common issues
- [ ] FAQ answers expected user questions
- [ ] Links between documents are correct
- [ ] No broken references or commands
- [ ] Migration checklist helps users track progress

---

## Success Criteria

Phase 11 is complete when:

1. ✅ Comprehensive migration guide exists at `docs/XVN_TO_ANVS_MIGRATION.md`
2. ✅ Migration script exists and works correctly
3. ✅ All documentation updated with migration links
4. ✅ Migration process tested successfully in clean environment
5. ✅ Common issues documented with solutions
6. ✅ Users have clear path from xvn to anvs
7. ✅ Both manual and automated migration options available

---

## Next Steps

After completing Phase 11:

1. **Review migration documentation** with fresh eyes (or ask someone to review)
2. **Test migration process** end-to-end in a VM or container
3. **Update any missing edge cases** in troubleshooting section
4. **Proceed to Phase 12**: Build, Test, and Publish

**Note**: Phase 12 will make the renamed package public. Ensure migration docs are complete and accurate before publishing!

---

## Rollback Plan

If issues are discovered in migration documentation:

1. **Before v2.0.0 publish**: Simply fix and update docs
2. **After v2.0.0 publish**:
   - Fix documentation issues immediately
   - Publish patch version if code changes needed
   - Update migration guide with corrections
   - Post update in GitHub issues/discussions

---

## Notes

- Migration guide is **critical for user success** - be thorough and clear
- Test migration process yourself before expecting users to do it
- Assume users have varying levels of technical expertise
- Provide both quick and detailed migration paths
- Automated script is optional but highly appreciated by users
- Good migration docs reduce support burden significantly
- Update migration guide based on real user feedback after launch
