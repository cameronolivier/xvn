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
