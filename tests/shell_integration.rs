use std::process::Command;

#[test]
fn test_xvn_sh_passes_shellcheck() {
    // Validate xvn.sh with shellcheck
    let output = Command::new("shellcheck")
        .args(&["--shell=bash", "shell/xvn.sh"])
        .output();

    match output {
        Ok(output) => {
            assert!(
                output.status.success(),
                "shellcheck failed:\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
        }
        Err(e) => {
            eprintln!("Warning: shellcheck not available: {}", e);
            eprintln!("Install shellcheck for full test coverage");
        }
    }
}

#[test]
fn test_shell_script_execution() {
    // Run the bash test script
    let output = Command::new("bash")
        .arg("tests/shell/test_xvn_sh.sh")
        .output()
        .expect("Failed to run shell test script");

    assert!(
        output.status.success(),
        "Shell test script failed:\n{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_profile_detection_bash() {
    use std::fs;
    use tempfile::TempDir;
    use xvn::setup::Shell;

    let temp = TempDir::new().unwrap();
    let home = temp.path();

    // Create only .bashrc
    let bashrc = home.join(".bashrc");
    fs::write(&bashrc, "# bash config\n").unwrap();

    let profiles = Shell::Bash.profile_files(home);

    // Should include .bashrc first
    assert_eq!(profiles[0], bashrc);
}

#[test]
fn test_profile_detection_zsh() {
    use std::fs;
    use tempfile::TempDir;
    use xvn::setup::Shell;

    let temp = TempDir::new().unwrap();
    let home = temp.path();

    // Create only .zshrc
    let zshrc = home.join(".zshrc");
    fs::write(&zshrc, "# zsh config\n").unwrap();

    let profiles = Shell::Zsh.profile_files(home);

    // Should include .zshrc first
    assert_eq!(profiles[0], zshrc);
}
