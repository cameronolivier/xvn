# Migration Guide: Upgrading to xvn v1.2+

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
