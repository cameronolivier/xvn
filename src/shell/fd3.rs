use log::{debug, trace};
use std::io;
use std::os::unix::io::RawFd;

/// Writer for shell commands via file descriptor #3
///
/// This struct handles the file descriptor #3 protocol, which allows
/// a child process (xvn) to send commands to the parent shell for execution.
///
/// # Protocol
/// - The shell opens FD:3 when invoking xvn: `xvn activate <path> 3>&1 1>&2 2>&3`
/// - xvn writes commands to FD:3
/// - The shell captures FD:3 output and evaluates it: `eval "$commands"`
///
/// # Safety
/// Commands must be properly escaped to prevent command injection.
/// FD:3 is owned by the parent shell, so we use raw writes without taking ownership.
pub struct CommandWriter {
    fd: Option<RawFd>,
}

impl CommandWriter {
    /// Creates a new CommandWriter
    ///
    /// Attempts to open file descriptor 3. If FD:3 is not available,
    /// returns a CommandWriter that silently discards writes.
    ///
    /// # Returns
    /// Always returns Ok - a non-functional writer if FD:3 unavailable
    pub fn new() -> io::Result<Self> {
        const FD3: RawFd = 3;

        // Check if FD:3 is open by attempting to get file status
        let fd = if Self::is_fd_open(FD3) {
            debug!("File descriptor 3 is available");
            Some(FD3)
        } else {
            debug!("File descriptor 3 is not available, commands will be discarded");
            None
        };

        Ok(Self { fd })
    }

    /// Checks if a file descriptor is open
    ///
    /// Uses fcntl to check if the FD is valid without modifying it.
    fn is_fd_open(fd: RawFd) -> bool {
        // Try to get file status flags
        // If fcntl succeeds, the FD is open
        unsafe { libc::fcntl(fd, libc::F_GETFD) != -1 }
    }

    /// Writes a shell command to FD:3
    ///
    /// The command will be executed by the parent shell. Multiple commands
    /// can be written; they will be concatenated with newlines.
    ///
    /// # Arguments
    /// * `command` - The shell command to write (must not contain newlines)
    ///
    /// # Security
    /// The caller must ensure the command is properly escaped to prevent
    /// command injection vulnerabilities.
    ///
    /// # Returns
    /// - Ok(()) if write successful or FD:3 not available
    /// - Err(io::Error) if write failed
    pub fn write_command(&mut self, command: &str) -> io::Result<()> {
        trace!("Writing command to FD:3: {command}");

        if let Some(fd) = self.fd {
            let data = format!("{command}\n");
            unsafe {
                let bytes_written =
                    libc::write(fd, data.as_ptr() as *const libc::c_void, data.len());
                if bytes_written == -1 {
                    return Err(io::Error::last_os_error());
                }
            }
        } else {
            trace!("FD:3 not available, command discarded");
        }

        Ok(())
    }

    /// Checks if FD:3 is available
    ///
    /// # Returns
    /// true if commands will be written to FD:3, false if they'll be discarded
    pub fn is_available(&self) -> bool {
        self.fd.is_some()
    }
}

impl Default for CommandWriter {
    fn default() -> Self {
        Self::new().expect("CommandWriter::new should never fail")
    }
}

/// Mock command writer for testing
#[cfg(test)]
pub struct MockCommandWriter {
    pub commands: Vec<String>,
}

#[cfg(test)]
impl Default for MockCommandWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
impl MockCommandWriter {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn write_command(&mut self, cmd: &str) -> std::io::Result<()> {
        self.commands.push(cmd.to_string());
        Ok(())
    }

    pub fn is_available(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::io::AsRawFd;
    use tempfile::NamedTempFile;

    #[test]
    fn test_command_writer_creation() {
        // CommandWriter should be created successfully regardless of FD:3 state
        let writer = CommandWriter::new();
        assert!(writer.is_ok());
    }

    #[test]
    #[ignore] // Skip this test (conflicts with tarpaulin FD handling during coverage)
    fn test_command_writer_with_fd3() {
        // Save FD:3 state if it's open
        let fd3_saved = unsafe {
            let dup_fd = libc::dup(3);
            if dup_fd != -1 {
                Some(dup_fd)
            } else {
                None
            }
        };

        // Create a temporary file and assign it to FD:3
        let temp = NamedTempFile::new().unwrap();
        let fd = temp.as_file().as_raw_fd();

        // Duplicate to FD:3
        unsafe {
            libc::dup2(fd, 3);
        }

        let mut writer = CommandWriter::new().unwrap();
        assert!(writer.is_available());

        // Write a command
        writer.write_command("echo test").unwrap();

        // Drop writer before restoring FD:3
        drop(writer);

        // Restore FD:3 or close it
        unsafe {
            if let Some(saved_fd) = fd3_saved {
                libc::dup2(saved_fd, 3);
                libc::close(saved_fd);
            } else {
                libc::close(3);
            }
        }

        // Keep temp alive until after FD:3 is restored
        drop(temp);
    }

    #[test]
    fn test_write_command_never_panics() {
        // write_command should never panic, even if FD:3 is invalid
        // If FD:3 is not available, it should succeed (silent discard)
        // If FD:3 is available but write fails, it returns an error (doesn't panic)
        let mut writer = CommandWriter::new().unwrap();

        // This should not panic - either succeeds or returns error
        let result = writer.write_command("echo test");

        // We can't assert success/failure because FD:3 state depends on test order
        // Just verify it returns a Result and doesn't panic
        let _ = result;
    }
}
