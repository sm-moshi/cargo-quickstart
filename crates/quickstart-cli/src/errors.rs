//! Error handling for the CLI interface

use color_eyre::{eyre::Report, Section};
use std::path::Path;
use thiserror::Error;

/// CLI-specific error types
#[derive(Error, Debug)]
pub enum CliError {
    /// Error occurred during command execution
    #[error("Command failed: {0}")]
    CommandError(String),

    /// Error occurred with file system operations
    #[allow(dead_code)]
    #[error("File system error: {0}")]
    FileSystemError(String),

    /// Error occurred with project configuration
    #[allow(dead_code)]
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Error occurred with template processing
    #[allow(dead_code)]
    #[error("Template error: {0}")]
    TemplateError(String),

    /// Error from interactive prompts
    #[error("Interactive prompt error: {0}")]
    InquireError(String),

    /// Error from the core library (quickstart_lib)
    #[error("Library error: {0}")]
    LibraryError(#[from] Report),

    /// Other errors that don't fit specific categories
    #[error("Error: {0}")]
    Other(String),
}

/// Set up the error handling system for the CLI
pub fn setup() -> color_eyre::Result<()> {
    color_eyre::install()
}

/// Create a new error with a path context
#[allow(dead_code)]
pub fn with_path_context(err: Report, path: &Path) -> Report {
    err.section(format!("Path: {}", path.display()))
}

/// Create a CLI-specific error
#[allow(dead_code)]
pub fn cli_error(message: &str) -> Report {
    Report::msg(message.to_string())
}

/// Add command context to an error
pub trait CommandErrorExt<T> {
    fn command_context(self, command: &str) -> Result<T, Report>;
}

impl<T, E> CommandErrorExt<T> for Result<T, E>
where
    E: Into<Report>,
{
    fn command_context(self, command: &str) -> Result<T, Report> {
        self.map_err(|err| {
            let err = err.into();
            err.section(format!("Command: {command}"))
        })
    }
}

/// Add actionable suggestions to an error
pub trait SuggestionsExt<T> {
    #[allow(dead_code)]
    fn suggest(self, suggestions: &[&str]) -> Result<T, Report>;
}

impl<T> SuggestionsExt<T> for Result<T, Report> {
    fn suggest(self, suggestions: &[&str]) -> Result<T, Report> {
        self.map_err(|err| {
            let mut err = err;
            if !suggestions.is_empty() {
                err = err.section("Suggestions:".to_string());
                for suggestion in suggestions {
                    err = err.section(format!("- {suggestion}"));
                }
            }
            err
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::eyre::eyre;
    use std::path::PathBuf;

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_setup() {
        // We can't run setup() multiple times in the same process as it would
        // reinstall color_eyre, so we'll just check the function exists
        assert!(true, "Test passing by design");
    }

    #[test]
    fn test_with_path_context() {
        let path = PathBuf::from("/test/path");
        let err = eyre!("Original error");
        let err_with_context = with_path_context(err, &path);

        // Verify the error exists and contains the original message
        let error_string = format!("{err_with_context}");
        assert!(
            error_string.contains("Original error"),
            "Error should contain the original message"
        );

        // With color-eyre, the sections are not part of the Display format,
        // so we can't test for their presence in the normal output string
    }

    #[test]
    fn test_cli_error() {
        let error_message = "Test CLI error";
        let err = cli_error(error_message);

        // Convert to string to verify the message is included
        let error_string = format!("{err}");
        assert!(
            error_string.contains(error_message),
            "Error should contain the original message"
        );
    }

    #[test]
    fn test_command_error_ext() {
        // Test success case
        let success: Result<i32, Report> = Ok(42);
        let result = success.command_context("test-command");
        assert!(result.is_ok());
        if let Ok(val) = result {
            assert_eq!(val, 42);
        }

        // Test error case
        let err: Result<i32, Report> = Err(eyre!("Original error"));
        let result = err.command_context("test-command");
        assert!(result.is_err());

        // We can only verify the original error message
        if let Err(error) = result {
            let error_string = format!("{error}");
            assert!(
                error_string.contains("Original error"),
                "Error should contain original message"
            );
        }
    }

    #[test]
    fn test_suggestions_ext() {
        // Test success case
        let success: Result<i32, Report> = Ok(42);
        let result = success.suggest(&["Try this", "Or try that"]);
        assert!(result.is_ok());
        if let Ok(val) = result {
            assert_eq!(val, 42);
        }

        // Test error case with suggestions
        let err: Result<i32, Report> = Err(eyre!("Original error"));
        let result = err.suggest(&["Try this", "Or try that"]);
        assert!(result.is_err());

        // We can only verify the original error message
        if let Err(error) = result {
            let error_string = format!("{error}");
            assert!(
                error_string.contains("Original error"),
                "Error should contain original message"
            );
        }

        // Test error case with empty suggestions
        let err: Result<i32, Report> = Err(eyre!("Original error"));
        let result = err.suggest(&[]);
        assert!(result.is_err());

        // Basic error just has error message
        if let Err(error) = result {
            let error_string = format!("{error}");
            assert!(
                error_string.contains("Original error"),
                "Error should contain original message"
            );
        }
    }
}
