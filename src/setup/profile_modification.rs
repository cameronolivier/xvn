use anyhow::{Context, Result};
use log::debug;
use std::fs;
use std::path::Path;

const ANVS_MARKER_START: &str = "# >>> anvs initialize >>>";
const ANVS_MARKER_END: &str = "# <<< anvs initialize <<<";

/// Adds or updates anvs initialization in a profile file.
/// If an old anvs block is found, it is replaced.
pub fn add_to_profile(profile: &Path) -> Result<()> {
    debug!("Updating anvs config in profile: {}", profile.display());

    // Always remove any existing block first to ensure a clean slate.
    let mut content = if profile.exists() {
        let current_content = fs::read_to_string(profile)
            .with_context(|| format!("Failed to read profile: {}", profile.display()))?;
        remove_anvs_block(&current_content)
    } else {
        String::new()
    };

    // Ensure content ends with a newline before adding our block.
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }

    // Define the new setup block
    let setup_lines = r###"# anvs shell integration
export ANVS_DIR="$HOME/.anvs"
export PATH="$ANVS_DIR/bin:$PATH"

# Try npm installation location first
if [ -s "$ANVS_DIR/current/lib/anvs.sh" ]; then
  . "$ANVS_DIR/current/lib/anvs.sh"
# Try Homebrew installation location
elif command -v brew >/dev/null 2>&1 && [ -s "$(brew --prefix anvs 2>/dev/null)/lib/anvs.sh" ]; then
  . "$(brew --prefix anvs)/lib/anvs.sh"
fi
"###;

    // Add the new block
    content.push('\n');
    content.push_str(ANVS_MARKER_START);
    content.push('\n');
    content.push_str(setup_lines.trim());
    content.push('\n');
    content.push_str(ANVS_MARKER_END);
    content.push('\n');

    // Write back to the profile file
    fs::write(profile, content)
        .with_context(|| format!("Failed to write profile: {}", profile.display()))
}

/// Remove anvs block from a profile file
/// Returns Ok(true) if block was found and removed, Ok(false) if not found
pub fn remove_from_profile(profile: &Path) -> Result<bool> {
    if !profile.exists() {
        return Ok(false);
    }

    let content = fs::read_to_string(profile)
        .with_context(|| format!("Failed to read profile: {}", profile.display()))?;

    if !content.contains(ANVS_MARKER_START) {
        return Ok(false);
    }

    let new_content = remove_anvs_block(&content);

    fs::write(profile, new_content)
        .with_context(|| format!("Failed to write profile: {}", profile.display()))?;

    Ok(true)
}

/// Removes the anvs initialization block from a string content.
fn remove_anvs_block(content: &str) -> String {
    if !content.contains(ANVS_MARKER_START) {
        return content.to_string();
    }

    let start_idx = match content.find(ANVS_MARKER_START) {
        Some(idx) => idx,
        None => return content.to_string(),
    };

    let end_idx = match content.find(ANVS_MARKER_END) {
        Some(idx) => idx,
        None => return content.to_string(), // Should not happen if start is found
    };

    // Find the end of the end marker line
    let end_line_end = content[end_idx..]
        .find('\n')
        .map(|i| end_idx + i + 1)
        .unwrap_or(content.len());

    // Also remove preceding newline if present to avoid extra blank lines
    let actual_start = if start_idx > 0 && content.as_bytes().get(start_idx - 1) == Some(&b'\n') {
        start_idx - 1
    } else {
        start_idx
    };

    format!("{}{}", &content[..actual_start], &content[end_line_end..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs as std_fs;

    const OLD_SETUP_BLOCK: &str = r###"# >>> anvs initialize >>>
# anvs (Automatic Node Version Switcher) - Automatic Node.js version switching
if [ -s "/Users/user/.anvs/bin/anvs.sh" ]; then
    source "/Users/user/.anvs/bin/anvs.sh"
fi
# <<< anvs initialize <<<
"###;

    #[test]
    fn test_remove_block() {
        let content = format!("some content before{OLD_SETUP_BLOCK}and some after");
        let cleaned = remove_anvs_block(&content);
        assert!(!cleaned.contains(ANVS_MARKER_START));
        assert!(cleaned.contains("some content before"));
        assert!(cleaned.contains("and some after"));
    }

    #[test]
    fn test_add_to_profile_from_scratch() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        add_to_profile(&path).unwrap();
        let content = std_fs::read_to_string(&path).unwrap();

        assert!(content.contains(ANVS_MARKER_START));
        assert!(content.contains("export PATH"));
    }

    #[test]
    fn test_add_to_profile_migrates_old_block() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();
        std_fs::write(&path, OLD_SETUP_BLOCK).unwrap();

        add_to_profile(&path).unwrap();
        let content = std_fs::read_to_string(&path).unwrap();

        // Check that new content is present
        assert!(content.contains("export PATH"));
        assert!(content.contains("[ -s \"$ANVS_DIR/current/lib/anvs.sh\" ]"));

        // Check that old content is gone
        assert!(!content.contains("if [ -s \"/Users/user/.anvs/bin/anvs.sh\" ]"));

        // Check that there is only one set of markers
        assert_eq!(content.matches(ANVS_MARKER_START).count(), 1);
    }
}
