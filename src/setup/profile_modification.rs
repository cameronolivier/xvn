use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use log::{debug, info};

const XVN_MARKER_START: &str = "# >>> xvn initialize >>>";
const XVN_MARKER_END: &str = "# <<< xvn initialize <<<";

/// Checks if a profile file already contains xvn initialization
pub fn is_already_installed(profile: &Path) -> Result<bool> {
    if !profile.exists() {
        return Ok(false);
    }

    let content = fs::read_to_string(profile)
        .with_context(|| format!("Failed to read profile: {}", profile.display()))?;

    Ok(content.contains(XVN_MARKER_START))
}

/// Adds xvn initialization to a profile file
///
/// The initialization block is wrapped in markers for idempotency checks.
///
/// # Arguments
/// * `profile` - Path to the profile file
/// * `xvn_sh_path` - Path to the xvn.sh script to source
pub fn add_to_profile(profile: &Path, xvn_sh_path: &Path) -> Result<()> {
    debug!("Adding xvn to profile: {}", profile.display());

    // Read existing content (or empty if file doesn't exist)
    let mut content = if profile.exists() {
        fs::read_to_string(profile)
            .with_context(|| format!("Failed to read profile: {}", profile.display()))?
    } else {
        String::new()
    };

    // Check if already installed
    if content.contains(XVN_MARKER_START) {
        info!("xvn already installed in {}", profile.display());
        return Ok(());
    }

    // Ensure content ends with newline
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }

    // Add initialization block
    content.push('\n');
    content.push_str(XVN_MARKER_START);
    content.push('\n');
    content.push_str("# xvn (Extreme Version Switcher) - Automatic Node.js version switching\n");
    content.push_str(&format!("if [ -s \"{}\" ]; then\n", xvn_sh_path.display()));
    content.push_str(&format!("    source \"{}\"\n", xvn_sh_path.display()));
    content.push_str("fi\n");
    content.push_str(XVN_MARKER_END);
    content.push('\n');

    // Write back
    fs::write(profile, content)
        .with_context(|| format!("Failed to write profile: {}", profile.display()))?;

    info!("Added xvn initialization to {}", profile.display());
    Ok(())
}

/// Removes xvn initialization from a profile file
///
/// Used for uninstallation (not in MVP, but defined for completeness)
#[allow(dead_code)]
pub fn remove_from_profile(profile: &Path) -> Result<()> {
    if !profile.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(profile)
        .with_context(|| format!("Failed to read profile: {}", profile.display()))?;

    if !content.contains(XVN_MARKER_START) {
        return Ok(());
    }

    // Remove everything between markers (including markers)
    let start_idx = content.find(XVN_MARKER_START).unwrap();
    let end_idx = content.find(XVN_MARKER_END)
        .context("Found start marker but not end marker")?;

    // Find the end of the end marker line
    let end_line_end = content[end_idx..].find('\n').unwrap_or(content.len() - end_idx) + end_idx + 1;

    // Also remove preceding newline if present
    let actual_start = if start_idx > 0 && content.as_bytes()[start_idx - 1] == b'\n' {
        start_idx - 1
    } else {
        start_idx
    };

    let new_content = format!("{}{}", &content[..actual_start], &content[end_line_end..]);

    fs::write(profile, new_content)
        .with_context(|| format!("Failed to write profile: {}", profile.display()))?;

    info!("Removed xvn initialization from {}", profile.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn test_is_already_installed() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "some content").unwrap();
        writeln!(temp, "{}", XVN_MARKER_START).unwrap();
        writeln!(temp, "source ~/.xvn/bin/xvn.sh").unwrap();
        writeln!(temp, "{}", XVN_MARKER_END).unwrap();
        temp.flush().unwrap();

        assert!(is_already_installed(temp.path()).unwrap());
    }

    #[test]
    fn test_add_to_profile() {
        let temp = NamedTempFile::new().unwrap();
        let xvn_sh = PathBuf::from("/home/user/.xvn/bin/xvn.sh");

        add_to_profile(temp.path(), &xvn_sh).unwrap();

        let content = fs::read_to_string(temp.path()).unwrap();
        assert!(content.contains(XVN_MARKER_START));
        assert!(content.contains(XVN_MARKER_END));
        assert!(content.contains("/home/user/.xvn/bin/xvn.sh"));
    }

    #[test]
    fn test_add_to_profile_idempotent() {
        let temp = NamedTempFile::new().unwrap();
        let xvn_sh = PathBuf::from("/home/user/.xvn/bin/xvn.sh");

        add_to_profile(temp.path(), &xvn_sh).unwrap();
        let content1 = fs::read_to_string(temp.path()).unwrap();

        // Add again
        add_to_profile(temp.path(), &xvn_sh).unwrap();
        let content2 = fs::read_to_string(temp.path()).unwrap();

        // Should be identical
        assert_eq!(content1, content2);
    }
}
