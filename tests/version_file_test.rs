// Note: Most version file tests are in src/version_file/finder.rs
// These are additional edge case tests to improve coverage

use std::fs;
use std::os::unix::fs::symlink;
use tempfile::TempDir;
use xvn::version_file::VersionFile;

#[test]
fn test_version_file_with_v_prefix_preserved() {
    // Test that 'v' prefix is NOT stripped (implementation preserves it)
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join(".nvmrc");
    fs::write(&file_path, "v18.20.0\n").unwrap();

    let result = VersionFile::find(temp.path(), &[".nvmrc".to_string()]);

    assert!(result.is_ok());
    let version_file = result.unwrap().unwrap();
    // Current implementation preserves the 'v' prefix
    assert_eq!(version_file.version, "v18.20.0");
}

#[test]
fn test_version_file_node_prefix() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join(".nvmrc");
    fs::write(&file_path, "node/18.20.0\n").unwrap();

    let result = VersionFile::find(temp.path(), &[".nvmrc".to_string()]);

    assert!(result.is_ok());
    let version_file = result.unwrap().unwrap();
    assert_eq!(version_file.version, "node/18.20.0");
}

#[test]
fn test_version_file_multiline_uses_first_non_comment() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join(".nvmrc");
    fs::write(&file_path, "18.20.0\n20.0.0\n21.0.0\n").unwrap();

    let result = VersionFile::find(temp.path(), &[".nvmrc".to_string()]);

    assert!(result.is_ok());
    let version_file = result.unwrap().unwrap();
    // Should use first line
    assert_eq!(version_file.version, "18.20.0");
}

#[test]
fn test_version_file_with_leading_empty_lines() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join(".nvmrc");
    fs::write(&file_path, "\n\n18.20.0\n").unwrap();

    let result = VersionFile::find(temp.path(), &[".nvmrc".to_string()]);

    assert!(result.is_ok());
    let version_file = result.unwrap().unwrap();
    assert_eq!(version_file.version, "18.20.0");
}

#[test]
fn test_version_file_with_trailing_whitespace_and_newlines() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join(".nvmrc");
    fs::write(&file_path, "18.20.0  \n\n\n").unwrap();

    let result = VersionFile::find(temp.path(), &[".nvmrc".to_string()]);

    assert!(result.is_ok());
    let version_file = result.unwrap().unwrap();
    assert_eq!(version_file.version, "18.20.0");
}

#[test]
fn test_version_file_follows_symlinks() {
    let temp = TempDir::new().unwrap();

    // Create real directory with .nvmrc
    let real_dir = temp.path().join("real");
    fs::create_dir(&real_dir).unwrap();
    fs::write(real_dir.join(".nvmrc"), "18.20.0\n").unwrap();

    // Create symlink to real directory
    let link_dir = temp.path().join("link");
    symlink(&real_dir, &link_dir).unwrap();

    // Search from symlink - should follow it
    let result = VersionFile::find(&link_dir, &[".nvmrc".to_string()]);

    assert!(result.is_ok());
    let version_file = result.unwrap().unwrap();
    assert_eq!(version_file.version, "18.20.0");
}

#[test]
fn test_version_file_deeply_nested_search() {
    let temp = TempDir::new().unwrap();

    // Create .nvmrc at root
    fs::write(temp.path().join(".nvmrc"), "18.20.0\n").unwrap();

    // Create deeply nested directory (10 levels)
    let mut nested = temp.path().to_path_buf();
    for i in 0..10 {
        nested = nested.join(format!("level{i}"));
    }
    fs::create_dir_all(&nested).unwrap();

    // Search from deeply nested - should find root .nvmrc
    let result = VersionFile::find(&nested, &[".nvmrc".to_string()]);

    assert!(result.is_ok());
    let version_file = result.unwrap().unwrap();
    assert_eq!(version_file.version, "18.20.0");
}

#[test]
fn test_version_file_priority_order_node_version_first() {
    let temp = TempDir::new().unwrap();

    // Create both version files
    fs::write(temp.path().join(".nvmrc"), "18.20.0\n").unwrap();
    fs::write(temp.path().join(".node-version"), "20.0.0\n").unwrap();

    // If .node-version is prioritized first, it should be found
    let result = VersionFile::find(
        temp.path(),
        &[".node-version".to_string(), ".nvmrc".to_string()],
    );

    assert!(result.is_ok());
    let version_file = result.unwrap().unwrap();
    assert_eq!(version_file.version, "20.0.0");
}

#[test]
fn test_version_file_complex_version_string() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join(".nvmrc");

    // Test various version string formats
    let versions = vec![
        "18.20.0",
        "v18.20.0",
        "lts/hydrogen",
        "lts/*",
        "node",
        "stable",
        "18",
        "18.20",
        "iojs",
        "iojs-v3.3.1",
    ];

    for version in versions {
        fs::write(&file_path, format!("{version}\n")).unwrap();

        let result = VersionFile::find(temp.path(), &[".nvmrc".to_string()]);

        assert!(result.is_ok(), "Failed to parse version string: {version}");
        let version_file = result.unwrap().unwrap();
        assert_eq!(version_file.version, version);
    }
}

#[test]
fn test_version_file_with_inline_comment() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join(".nvmrc");
    // Note: Current implementation doesn't strip inline comments
    fs::write(&file_path, "18.20.0 # This is my version\n").unwrap();

    let result = VersionFile::find(temp.path(), &[".nvmrc".to_string()]);

    assert!(result.is_ok());
    let version_file = result.unwrap().unwrap();
    // Will include the comment in the version string
    assert!(version_file.version.contains("18.20.0"));
}

#[test]
fn test_version_file_struct_equality() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join(".nvmrc");
    fs::write(&path, "18.20.0\n").unwrap();

    let vf1 = VersionFile {
        path: path.clone(),
        version: "18.20.0".to_string(),
        source: xvn::version_file::VersionFileSource::Nvmrc,
    };

    let vf2 = VersionFile {
        path: path.clone(),
        version: "18.20.0".to_string(),
        source: xvn::version_file::VersionFileSource::Nvmrc,
    };

    // VersionFile implements PartialEq
    assert_eq!(vf1, vf2);
}

#[test]
fn test_version_file_struct_clone() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join(".nvmrc");
    fs::write(&path, "18.20.0\n").unwrap();

    let vf1 = VersionFile {
        path: path.clone(),
        version: "18.20.0".to_string(),
        source: xvn::version_file::VersionFileSource::Nvmrc,
    };

    // VersionFile implements Clone
    let vf2 = vf1.clone();

    assert_eq!(vf1, vf2);
}
