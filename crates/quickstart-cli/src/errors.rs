//! Error handling for the CLI interface

use color_eyre::{eyre::Report, Section};
use std::path::Path;

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
