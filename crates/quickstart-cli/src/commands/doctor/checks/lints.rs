//! Check for linting issues in the project

use crate::commands::doctor::diagnosis::Check;
use crate::commands::doctor::types::{Diagnostic, Severity};
use std::process::{Command, Output};

/// Trait for executing clippy command, allowing for easier mocking in tests
pub trait CommandExecutor {
    fn execute_clippy(&self) -> std::io::Result<Output>;
}

/// Default implementation that calls the actual cargo clippy command
pub struct RealCommandExecutor;

impl CommandExecutor for RealCommandExecutor {
    fn execute_clippy(&self) -> std::io::Result<Output> {
        Command::new("cargo")
            .args(["clippy", "--quiet", "--message-format=json"])
            .output()
    }
}

/// Check for linting issues in the project
pub struct LintsCheck {
    executor: Box<dyn CommandExecutor>,
}

impl LintsCheck {
    /// Create a new LintsCheck with the default command executor
    pub fn new() -> Self {
        Self {
            executor: Box::new(RealCommandExecutor),
        }
    }

    /// Create a new LintsCheck with a custom command executor (for testing)
    #[cfg(test)]
    pub fn with_executor(executor: Box<dyn CommandExecutor>) -> Self {
        Self { executor }
    }
}

impl Check for LintsCheck {
    fn run(&self) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Try to run cargo clippy
        let clippy_result = self.executor.execute_clippy();

        match clippy_result {
            Ok(output) => {
                if output.status.success() {
                    diagnostics.push(Diagnostic::new(
                        self.name(),
                        Severity::Info,
                        "No linting issues found with clippy",
                        self.category(),
                    ));
                } else {
                    // Parse output for issues
                    diagnostics.push(
                        Diagnostic::new(
                            self.name(),
                            Severity::Warning,
                            "Clippy found linting issues",
                            self.category(),
                        )
                        .with_suggestion("Run 'cargo clippy' to see and fix the issues"),
                    );
                }
            }
            Err(_) => {
                diagnostics.push(
                    Diagnostic::new(
                        self.name(),
                        Severity::Warning,
                        "Failed to run cargo clippy",
                        self.category(),
                    )
                    .with_suggestion(
                        "Make sure clippy is installed: 'rustup component add clippy'",
                    ),
                );
            }
        }

        diagnostics
    }

    fn name(&self) -> &str {
        "Linting"
    }

    fn description(&self) -> &str {
        "Check for linting issues with clippy"
    }

    fn category(&self) -> &str {
        "lints"
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;

    // Mock command executor for testing different scenarios
    enum MockExecutorResult {
        Success(Output),
        Failure(Output),
        Error,
    }

    struct MockCommandExecutor {
        result: MockExecutorResult,
    }

    impl CommandExecutor for MockCommandExecutor {
        fn execute_clippy(&self) -> std::io::Result<Output> {
            match &self.result {
                MockExecutorResult::Success(output) => Ok(output.clone()),
                MockExecutorResult::Failure(output) => Ok(output.clone()),
                MockExecutorResult::Error => {
                    Err(Error::new(ErrorKind::NotFound, "command not found"))
                }
            }
        }
    }

    // Helper to create a success output
    fn success_output() -> Output {
        Output {
            status: ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }

    // Helper to create a failure output
    fn failure_output() -> Output {
        Output {
            status: ExitStatus::from_raw(1),
            stdout: Vec::new(),
            stderr: b"clippy errors found".to_vec(),
        }
    }

    #[test]
    fn test_lints_check_success() {
        let executor = MockCommandExecutor {
            result: MockExecutorResult::Success(success_output()),
        };
        let check = LintsCheck::with_executor(Box::new(executor));

        let diagnostics = check.run();

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, Severity::Info);
        assert!(diagnostics[0].message.contains("No linting issues found"));
    }

    #[test]
    fn test_lints_check_warning() {
        let executor = MockCommandExecutor {
            result: MockExecutorResult::Failure(failure_output()),
        };
        let check = LintsCheck::with_executor(Box::new(executor));

        let diagnostics = check.run();

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, Severity::Warning);
        assert!(diagnostics[0]
            .message
            .contains("Clippy found linting issues"));
        assert!(diagnostics[0]
            .suggestion
            .as_ref()
            .unwrap()
            .contains("Run 'cargo clippy'"));
    }

    #[test]
    fn test_lints_check_error() {
        let executor = MockCommandExecutor {
            result: MockExecutorResult::Error,
        };
        let check = LintsCheck::with_executor(Box::new(executor));

        let diagnostics = check.run();

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, Severity::Warning);
        assert!(diagnostics[0]
            .message
            .contains("Failed to run cargo clippy"));
        assert!(diagnostics[0]
            .suggestion
            .as_ref()
            .unwrap()
            .contains("Make sure clippy is installed"));
    }

    #[test]
    fn test_name_method() {
        let check = LintsCheck::new();
        assert_eq!(check.name(), "Linting");
    }

    #[test]
    fn test_description_method() {
        let check = LintsCheck::new();
        assert_eq!(check.description(), "Check for linting issues with clippy");
    }

    #[test]
    fn test_category_method() {
        let check = LintsCheck::new();
        assert_eq!(check.category(), "lints");
    }
}
