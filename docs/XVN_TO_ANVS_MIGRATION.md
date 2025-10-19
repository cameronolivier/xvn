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
