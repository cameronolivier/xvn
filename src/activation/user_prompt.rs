use std::io::{self, Write};

/// Abstraction for user prompts (allows testing without stdin)
pub trait UserPrompt {
    /// Ask user a yes/no question
    /// Returns: true if user confirms, false if user declines
    fn confirm(&mut self, message: &str) -> io::Result<bool>;
}

/// Production implementation that reads from stdin
pub struct StdinUserPrompt {
    stdin: io::Stdin,
    stdout: io::Stdout,
}

impl StdinUserPrompt {
    pub fn new() -> Self {
        Self {
            stdin: io::stdin(),
            stdout: io::stdout(),
        }
    }
}

impl Default for StdinUserPrompt {
    fn default() -> Self {
        Self::new()
    }
}

impl UserPrompt for StdinUserPrompt {
    fn confirm(&mut self, message: &str) -> io::Result<bool> {
        // Print prompt
        write!(self.stdout, "{message} [Y/n]: ")?;
        self.stdout.flush()?;

        // Read response
        let mut response = String::new();
        self.stdin.read_line(&mut response)?;

        // Parse response (default to "yes" if just Enter pressed)
        let response = response.trim().to_lowercase();
        Ok(response.is_empty() || response.starts_with('y'))
    }
}

/// Mock implementation for testing
#[cfg(test)]
pub struct MockUserPrompt {
    pub responses: Vec<bool>,
    pub prompts_received: Vec<String>,
}

#[cfg(test)]
impl MockUserPrompt {
    pub fn new(responses: Vec<bool>) -> Self {
        Self {
            responses,
            prompts_received: Vec::new(),
        }
    }
}

#[cfg(test)]
impl UserPrompt for MockUserPrompt {
    fn confirm(&mut self, message: &str) -> io::Result<bool> {
        self.prompts_received.push(message.to_string());
        Ok(self.responses.pop().unwrap_or(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_user_prompt_yes() {
        let mut prompt = MockUserPrompt::new(vec![true]);
        let result = prompt.confirm("Install Node.js?").unwrap();
        assert!(result);
        assert_eq!(prompt.prompts_received.len(), 1);
        assert!(prompt.prompts_received[0].contains("Install Node.js?"));
    }

    #[test]
    fn test_mock_user_prompt_no() {
        let mut prompt = MockUserPrompt::new(vec![false]);
        let result = prompt.confirm("Install Node.js?").unwrap();
        assert!(!result);
    }

    #[test]
    fn test_mock_user_prompt_multiple() {
        // Note: responses are popped in reverse order
        let mut prompt = MockUserPrompt::new(vec![false, true]);
        assert!(prompt.confirm("First?").unwrap());
        assert!(!prompt.confirm("Second?").unwrap());
        assert_eq!(prompt.prompts_received.len(), 2);
    }

    #[test]
    fn test_mock_user_prompt_default_false() {
        let mut prompt = MockUserPrompt::new(vec![]);
        let result = prompt.confirm("Install?").unwrap();
        assert!(!result); // Default to false when no responses
    }
}
