# XVN Has Been Renamed to ANVS

⚠️ **IMPORTANT NOTICE**: This project has been renamed to **anvs** (Automatic Node Version Switcher)

## Why the Rename?

- **Better Name**: "Automatic Node Version Switcher" is more descriptive
- **Unnamespaced Package**: `anvs` is available without namespace (vs `@olvrcc/xvn`)
- **Clearer Purpose**: Name immediately communicates what the tool does
- **Tribute to avn**: Pays homage to the original project while being distinct

## What's Changing?

| Old (xvn)                | New (anvs)               |
|--------------------------|--------------------------|
| `@olvrcc/xvn`            | `anvs`                   |
| `xvn` binary             | `anvs` binary            |
| `~/.xvn/` directory      | `~/.anvs/` directory     |
| `~/.xvnrc` config        | `~/.anvsrc` config       |
| `.xvn.yaml` project file | `.anvs.yaml` project file|

## Migration Instructions

### For New Users
Install the new package directly:
```bash
npm install -g anvs
anvs setup
```

### For Existing XVN Users

1. **Backup your config** (optional but recommended):
   ```bash
   cp ~/.xvnrc ~/.xvnrc.backup
   ```

2. **Uninstall xvn**:
   ```bash
   xvn uninstall
   # or manually: npm uninstall -g @olvrcc/xvn
   ```

3. **Install anvs**:
   ```bash
   npm install -g anvs
   anvs setup
   ```

4. **Migrate config** (if you had custom settings):
   - Copy settings from `~/.xvnrc.backup` to `~/.anvsrc`
   - Rename project-level `.xvn.yaml` to `.anvs.yaml`

## Timeline

- **v1.7.0** (Current): Final xvn release with deprecation notice
- **v2.0.0**: New `anvs` package published (coming soon)
- **Ongoing**: Both packages will coexist during transition period

## Support

- New package: https://github.com/olvrcc/anvs
- Migration help: https://github.com/olvrcc/anvs/issues
- Full migration guide: [Link will be added when published]

## Will XVN Stop Working?

**No!** Your current `xvn` installation will continue to work indefinitely. However:
- No new features will be added to `xvn`
- Bug fixes and updates will only go to `anvs`
- We recommend migrating when convenient

---

*Thank you for using xvn! We look forward to seeing you on anvs.*
