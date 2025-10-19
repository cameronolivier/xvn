# Migration Guide

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

## Upgrading to anvs v2.1.0 (Wizard Redesign)

Version 2.1.0 introduces a redesigned `anvs init` wizard with improved user experience and faster setup. This is a **non-breaking upgrade** - existing configurations continue to work.

### What's New in v2.1.0

- **Quick Mode** (default): Auto-detection with single confirmation, completes in < 30 seconds
- **Advanced Mode** (`--advanced`): Step-by-step customization for full control
- **Visual Improvements**: Timeline-style progress indicators and modern UI
- **Better Error Handling**: Clearer messages when detection fails

### Upgrade Instructions

```bash
# Update to v2.1.0
npm update -g anvs

# Or reinstall
npm install -g anvs

# Run the new wizard (optional - existing config still works)
anvs init
```

### New Wizard Behavior

**Default behavior changed**: `anvs init` now runs quick mode by default. For the old full customization experience, use:

```bash
anvs init --advanced
```

### Rollback (If Needed)

If you prefer the old wizard experience:

```bash
# Downgrade to v2.0.0
npm install -g anvs@2.0.0

# Re-run setup
anvs init
```

---

## Migrating from xvn to anvs (v1.x → v2.0) - Detailed Instructions

**IMPORTANT**: This is a breaking change. The project has been renamed from `xvn` to `anvs`.

### Why the Rename?

- **Better package name**: `anvs` is available unnamespaced on npm (vs `@olvrcc/xvn`)
- **Clearer purpose**: "Automatic Node Version Switcher" immediately communicates what the tool does
- **Improved discoverability**: Easier to find and remember
- **Tribute to avn**: Pays homage to the original project while being distinct

### What Changed

| Component | Old (xvn) | New (anvs) |
|-----------|-----------|------------|
| npm package | `@olvrcc/xvn` | `anvs` |
| Binary name | `xvn` | `anvs` |
| Install directory | `~/.xvn/` | `~/.anvs/` |
| User config | `~/.xvnrc` | `~/.anvsrc` |
| Project config | `.xvn.yaml` | `.anvs.yaml` |
| Shell script | `shell/xvn.sh` | `shell/anvs.sh` |
| Environment vars | `XVN_*` | `ANVS_*` |
| Repository | `github.com/olvrcc/xvn` | `github.com/olvrcc/anvs` |
| Homebrew tap | `olvrcc/xvn` | `olvrcc/anvs` |

### Migration Steps

#### Step 1: Backup Your Configuration

```bash
# Backup your xvn config (optional but recommended)
cp ~/.xvnrc ~/.xvnrc.backup

# Backup project-level config files if you use them
find . -name ".xvn.yaml" -exec cp {} {}.backup \;
```

#### Step 2: Uninstall Old xvn

```bash
# Option 1: Use xvn's uninstall command (recommended)
xvn uninstall

# Option 2: Manual uninstall
npm uninstall -g @olvrcc/xvn

# Remove shell integration from ~/.bashrc or ~/.zshrc
# Look for and remove lines like:
# [ -s "$HOME/.xvn/bin/xvn.sh" ] && . "$HOME/.xvn/bin/xvn.sh"

# Optionally remove old installation directory
rm -rf ~/.xvn
```

#### Step 3: Install New anvs

```bash
# Via npm (recommended)
npm install -g anvs

# Or via Homebrew
brew install olvrcc/anvs/anvs

# Run setup to configure shell integration
anvs setup
```

#### Step 4: Migrate Configuration

**Option A: Copy your old config** (if you had custom settings):
```bash
# Copy contents from backup to new config file
cp ~/.xvnrc.backup ~/.anvsrc

# Or manually edit
nano ~/.anvsrc
```

**Option B: Start fresh** (if you used defaults):
```bash
# anvs will create a default config on first run
# Customize with:
anvs set config.default_version
anvs set plugins.priority
```

**Project-level config files**:
```bash
# Rename .xvn.yaml files to .anvs.yaml in your projects
# In each project directory:
mv .xvn.yaml .anvs.yaml

# Or use find to batch rename:
find ~/projects -name ".xvn.yaml" -execdir mv {} .anvs.yaml \;
```

#### Step 5: Reload Shell

```bash
# Reload your shell configuration
source ~/.bashrc  # or source ~/.zshrc

# Verify anvs is loaded
which anvs
# Should show: /Users/yourusername/.anvs/bin/anvs
```

#### Step 6: Verify Installation

```bash
# Check version
anvs --version
# Should show: anvs 2.0.0

# Check status
anvs status

# Test activation in a project with .nvmrc
cd your-project
# Should auto-activate the Node.js version
```

### Troubleshooting Migration

#### Shell integration not working

```bash
# Check if shell script is sourced
grep -i "anvs" ~/.bashrc ~/.zshrc

# Should see line like:
# [ -s "$HOME/.anvs/bin/anvs.sh" ] && . "$HOME/.anvs/bin/anvs.sh"

# If missing, run setup again
anvs setup
```

#### Old xvn references remain in shell

```bash
# Check for and remove old xvn references
grep -i "xvn" ~/.bashrc ~/.zshrc

# Manually remove any lines containing .xvn
# Then reload shell
source ~/.bashrc  # or source ~/.zshrc
```

#### Config not loading

```bash
# Verify config file exists
ls -la ~/.anvsrc

# Check config syntax
cat ~/.anvsrc

# Test with explicit config path
ANVS_DEBUG=1 anvs status
```

#### Both xvn and anvs installed

```bash
# Check what's in your PATH
which xvn
which anvs

# If both present, make sure anvs takes precedence
# Or fully uninstall xvn:
npm uninstall -g @olvrcc/xvn
rm -rf ~/.xvn
```

### Rollback (If Needed)

If you need to go back to xvn:

```bash
# Uninstall anvs
anvs uninstall
npm uninstall -g anvs

# Reinstall old xvn
npm install -g @olvrcc/xvn@1.7.0
xvn setup

# Restore backup config
cp ~/.xvnrc.backup ~/.xvnrc
```

### FAQ

**Q: Will my old xvn installation stop working?**
A: No, existing installations continue to work. However, xvn won't receive updates.

**Q: Can I keep both xvn and anvs installed?**
A: Not recommended. They use similar shell hooks and may conflict. Choose one.

**Q: Do I need to update my project's .nvmrc files?**
A: No, .nvmrc files remain unchanged. anvs reads the same version files as xvn.

**Q: What about my nvm/fnm installation?**
A: No changes needed. anvs works with your existing version manager.

**Q: Is there an automatic migration script?**
A: Not currently. Manual migration ensures you understand the changes.

**Q: Where can I get help?**
A: Open an issue at https://github.com/olvrcc/anvs/issues

---

## Upgrading to xvn v1.2+ (Historical)

*Note: The section below is for historical reference only. For current migrations, see the xvn→anvs section above.*

This guide explains how to migrate from an older version of `xvn` (v1.1.x and below) to the new version-independent installation (v1.2.0 and above).

## What's Changed?

The most critical change in v1.2.0 is **where `xvn` is installed**.

-   **Old way (<= v1.1.x):** `xvn` was installed into the global `node_modules` directory of your *currently active* Node.js version. This caused `xvn` to become unavailable when you switched to a different Node.js version.
-   **New way (>= v1.2.0):** `xvn` is now installed into a dedicated, version-independent directory at `~/.xvn`. The main binary is available at `~/.xvn/bin`, which is added to your shell's `PATH`.

This change fixes the biggest issue with `xvn`, ensuring it is **always available**, no matter which version of Node.js you are using.

## How to Upgrade

Upgrading is designed to be as seamless as possible.

### Step 1: Update the npm Package

Run the following command to update to the latest version:

```bash
npm install -g @olvrcc/xvn
```

The new post-installation script will automatically:
1.  Create the `~/.xvn` directory structure.
2.  Install the latest `xvn` binary into `~/.xvn/versions/`.
3.  Create a symlink to make it the `current` version.

### Step 2: Update Your Shell Configuration

After the package is updated, you need to update your shell configuration to use the new installation location. Run the `setup` command:

```bash
xvn setup
```

This command will automatically find your shell profile (e.g., `~/.zshrc`, `~/.bashrc`) and replace the old `xvn` initialization block with the new one.

**Old Block (will be removed):**
```bash
# >>> xvn initialize >>>
# ... some old commands ...
# <<< xvn initialize <<<
```

**New Block (will be added):**
```bash
# >>> xvn initialize >>>
# xvn shell integration
export XVN_DIR="$HOME/.xvn"
export PATH="$XVN_DIR/bin:$PATH"
[ -s "$XVN_DIR/current/lib/xvn.sh" ] && . "$XVN_DIR/current/lib/xvn.sh"
# <<< xvn initialize <<<
```

### Step 3: Restart Your Shell

To apply the changes, you must either restart your terminal or source your profile file:

```bash
# For Zsh
source ~/.zshrc

# For Bash
source ~/.bashrc
```

### Step 4: Verify the Installation

Run the following command to check which `xvn` binary is being used:

```bash
which xvn
```

It should point to your home directory:

```
/Users/your-username/.xvn/bin/xvn
```

If it still points to a Node.js version path (e.g., `/Users/your-username/.nvm/versions/node/v20.11.0/bin/xvn`), your shell may have a cached path. Try opening a new terminal window.

## Manual Migration

If you prefer to make the changes manually:

1.  **Update the package:** `npm install -g @olvrcc/xvn`
2.  **Edit your shell profile:** Open `~/.zshrc`, `~/.bashrc`, or equivalent.
3.  **Replace the block:** Find the `# >>> xvn initialize >>>` block and replace its entire contents with the new block shown in Step 2 above.
4.  **Save and source:** Save the file and source it (e.g., `source ~/.zshrc`).

## Rollback Procedure

If you encounter any issues, you can roll back:

1.  **Install the previous version:**
    ```bash
    npm install -g @olvrcc/xvn@1.1.2
    ```
2.  **Re-run setup:**
    ```bash
    xvn setup
    ```
    The setup command from the older version will rewrite your shell profile back to the old format.
3.  **Restart your shell.**

## FAQ

**Q: Why is this change necessary?**

A: To fix the fundamental bootstrap problem where `xvn` would disappear after switching Node.js versions.

**Q: Will my configuration be preserved?**

A: Yes. `xvn` configuration is stored in `~/.xvnrc`, which is not affected by this upgrade.

**Q: Do I need to reinstall `xvn` in each Node.js version now?**

A: No. This change means you only need to install and manage one central copy of `xvn`.
