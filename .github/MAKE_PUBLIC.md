# Making the Repository Public - Checklist

Follow these steps to make the xvn repository public and ready for contributions.

## Pre-Flight Checklist

### 1. Review Sensitive Information
- [x] No API keys, tokens, or credentials in commit history ✅
- [x] No personal information in commits (only public email addresses) ✅
- [x] No internal company references ✅
- [x] Searched for sensitive strings: `git grep -i "api.key\|token\|password\|secret\|credential"` ✅

**Email addresses in commits:**
- `cameronolivier@gmail.com` (public) ✅
- `cameron.olivier@mohara.co` (work email - consider changing if needed)

**Note:** All references to tokens/secrets in the codebase are:
- GitHub Actions variables (standard practice)
- Documentation references
- No actual secrets committed ✅

### 2. Verify Documentation
- [x] README.md is complete and accurate
- [x] LICENSE file exists (MIT)
- [x] CONTRIBUTING.md exists
- [x] ROADMAP.md created
- [x] Issue templates created
- [x] Pull request template created

### 3. Code Quality
- [x] All tests passing
- [x] No clippy warnings
- [x] Code formatted (`cargo fmt`)
- [x] CI/CD working

### 4. Package Registry
- [x] npm package published: `@olvrcc/xvn`
- [ ] Package description accurate
- [ ] Package keywords set
- [ ] Package repository URL correct

## Making the Repository Public

### On GitHub Website

1. **Go to repository settings:**
   ```
   https://github.com/cameronolivier/xvn/settings
   ```

2. **Scroll to "Danger Zone" section at bottom**

3. **Click "Change visibility"**

4. **Select "Make public"**

5. **Type repository name to confirm:** `cameronolivier/xvn`

6. **Click "I understand, make this repository public"**

### Using GitHub CLI

```bash
gh repo edit cameronolivier/xvn --visibility public
```

## Post-Publication Steps

### 1. Add Repository Topics

Add relevant topics to help discoverability:

```bash
gh repo edit cameronolivier/xvn --add-topic rust
gh repo edit cameronolivier/xvn --add-topic nodejs
gh repo edit cameronolivier/xvn --add-topic version-manager
gh repo edit cameronolivier/xvn --add-topic nvm
gh repo edit cameronolivier/xvn --add-topic cli
gh repo edit cameronolivier/xvn --add-topic automation
gh repo edit cameronolivier/xvn --add-topic bash
gh repo edit cameronolivier/xvn --add-topic zsh
gh repo edit cameronolivier/xvn --add-topic powershell
```

Or via website: Settings → scroll to "Topics"

### 2. Enable Discussions (Optional)

```bash
gh repo edit cameronolivier/xvn --enable-discussions
```

Or via website: Settings → Features → Check "Discussions"

### 3. Configure Branch Protection

Protect the main branch:

1. Go to Settings → Branches
2. Add rule for `main` branch:
   - [x] Require pull request reviews before merging
   - [x] Require status checks to pass
   - [x] Require branches to be up to date
   - [x] Include administrators

### 4. Add README Badges

Add to top of README.md:

```markdown
# xvn - Extreme Version Switcher

[![CI](https://github.com/cameronolivier/xvn/workflows/Test/badge.svg)](https://github.com/cameronolivier/xvn/actions)
[![npm version](https://badge.fury.io/js/@olvrcc%2Fxvn.svg)](https://www.npmjs.com/package/@olvrcc/xvn)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
```

### 5. Create First "Help Wanted" Issues

Create issues for immediate contributions:

```bash
# Windows testing
gh issue create --title "Help Wanted: Windows Testing" \
  --body "We need Windows developers to test xvn. See ROADMAP.md#milestone-11" \
  --label "help-wanted,windows"

# Homebrew formula
gh issue create --title "Help Wanted: Homebrew Formula" \
  --body "Looking for someone to create a Homebrew formula. See spec/milestone-9/" \
  --label "help-wanted,distribution"
```

### 6. Announce

Consider announcing on:
- [ ] Twitter/X
- [ ] Reddit (r/rust, r/node)
- [ ] Dev.to
- [ ] Hacker News (Show HN)
- [ ] This Week in Rust newsletter

Sample announcement:

```
🚀 Announcing xvn v1.1 - Automatic Node.js version switching

Written in Rust for speed (2-3x faster than avn)
✅ Works with nvm, fnm
✅ Auto-installs missing versions
✅ macOS & Linux ready
🔨 Windows support in progress (contributors needed!)

npm install -g @olvrcc/xvn

https://github.com/cameronolivier/xvn
```

## Security

### Enable Security Features

1. **Dependabot:**
   - Settings → Security & analysis
   - Enable "Dependabot alerts"
   - Enable "Dependabot security updates"

2. **Code scanning (optional):**
   - Settings → Security & analysis
   - Enable "Code scanning"
   - Set up CodeQL

### Security Policy

Create `.github/SECURITY.md`:

```markdown
# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.1.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, email: cameronolivier@gmail.com

You should receive a response within 48 hours.
```

## Final Verification

- [ ] Repository is public
- [ ] All workflows running correctly
- [ ] Issues template working
- [ ] PR template working
- [ ] License visible on GitHub
- [ ] README renders correctly
- [ ] npm package links to repo
- [ ] All links in docs work

## Done!

Your repository is now public and ready for contributions! 🎉

Monitor:
- GitHub Issues for bug reports
- Pull Requests for contributions
- npm downloads for adoption
- GitHub Stars for popularity
