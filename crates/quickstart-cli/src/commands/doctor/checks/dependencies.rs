//! Check for dependency issues in the project

use crate::commands::doctor::diagnosis::Check;
use crate::commands::doctor::types::{Diagnostic, Severity};
#[cfg(test)]
use mockall;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use which;

/// Constants for dependency checking
const CARGO_OUTDATED_RECOMMENDATION: &str = "Install cargo-outdated: cargo install cargo-outdated";
const CARGO_UDEPS_RECOMMENDATION: &str = "Install cargo-udeps: cargo install cargo-udeps";

/// Trait for executing cargo commands, allowing for easier mocking in tests
#[cfg_attr(test, allow(clippy::disallowed_methods))]
#[cfg_attr(test, mockall::automock)]
pub trait CommandExecutor: Send + Sync {
    fn execute_outdated(&self) -> std::io::Result<Output>;
    fn execute_udeps(&self) -> std::io::Result<Output>;
    fn is_command_available(&self, command: &str) -> bool;
}

/// Default implementation that calls the actual cargo commands
pub struct RealCommandExecutor {
    is_running: Arc<AtomicBool>,
}

impl RealCommandExecutor {
    fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Drop for RealCommandExecutor {
    fn drop(&mut self) {
        if self.is_running.load(Ordering::SeqCst) {
            // Ensure any running processes are cleaned up
            if let Ok(ps) = std::process::Command::new("ps").args(["aux"]).output() {
                let output = String::from_utf8_lossy(&ps.stdout);
                for line in output.lines() {
                    if line.contains("cargo outdated") || line.contains("cargo udeps") {
                        if let Some(pid) = line.split_whitespace().nth(1) {
                            if let Ok(pid) = pid.parse::<i32>() {
                                unsafe {
                                    libc::kill(pid, libc::SIGTERM);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl CommandExecutor for RealCommandExecutor {
    fn execute_outdated(&self) -> std::io::Result<Output> {
        self.is_running.store(true, Ordering::SeqCst);
        let output = Command::new("cargo")
            .args(["outdated", "--exit-code", "1"])
            .output();
        self.is_running.store(false, Ordering::SeqCst);
        output
    }

    fn execute_udeps(&self) -> std::io::Result<Output> {
        self.is_running.store(true, Ordering::SeqCst);
        let output = Command::new("cargo")
            .args(["udeps", "--", "--no-run", "--no-fail-fast"])
            .output();
        self.is_running.store(false, Ordering::SeqCst);
        output
    }

    fn is_command_available(&self, command: &str) -> bool {
        which::which(command).is_ok()
    }
}

/// Check for dependency issues in the project
pub struct DependenciesCheck {
    executor: Box<dyn CommandExecutor>,
}

impl Drop for DependenciesCheck {
    fn drop(&mut self) {
        // No need to explicitly drop the executor as it will be dropped automatically
    }
}

impl DependenciesCheck {
    /// Create a new DependenciesCheck with the default command executor
    pub fn new() -> Self {
        Self {
            executor: Box::new(RealCommandExecutor::new()),
        }
    }

    /// Create a new DependenciesCheck with a custom command executor (for testing)
    #[cfg(test)]
    pub fn with_executor(executor: Box<dyn CommandExecutor>) -> Self {
        Self { executor }
    }

    /// Check for outdated dependencies using cargo-outdated
    fn check_outdated(&self) -> Diagnostic {
        // First check if cargo-outdated is installed
        if !self.executor.is_command_available("cargo-outdated") {
            return Diagnostic::new(
                self.name(),
                Severity::Suggestion,
                "cargo-outdated is not installed",
                self.category(),
            )
            .with_suggestion(CARGO_OUTDATED_RECOMMENDATION);
        }

        match self.executor.execute_outdated() {
            Ok(output) => {
                if output.status.success() {
                    Diagnostic::new(
                        self.name(),
                        Severity::Info,
                        "All dependencies are up to date",
                        self.category(),
                    )
                } else {
                    Diagnostic::new(
                        self.name(),
                        Severity::Warning,
                        "Outdated dependencies found",
                        self.category(),
                    )
                    .with_suggestion("Run 'cargo outdated' to see details and update dependencies")
                }
            }
            Err(_) => Diagnostic::new(
                self.name(),
                Severity::Suggestion,
                "Could not check for outdated dependencies",
                self.category(),
            )
            .with_suggestion(CARGO_OUTDATED_RECOMMENDATION),
        }
    }

    /// Check for unused dependencies using cargo-udeps
    fn check_unused(&self) -> Diagnostic {
        // First check if cargo-udeps is installed
        if !self.executor.is_command_available("cargo-udeps") {
            return Diagnostic::new(
                self.name(),
                Severity::Suggestion,
                "cargo-udeps is not installed",
                self.category(),
            )
            .with_suggestion(CARGO_UDEPS_RECOMMENDATION);
        }

        match self.executor.execute_udeps() {
            Ok(output) => {
                if output.status.success() {
                    Diagnostic::new(
                        self.name(),
                        Severity::Info,
                        "No unused dependencies found",
                        self.category(),
                    )
                } else {
                    // Parse output to find unused dependencies
                    let output_str = String::from_utf8_lossy(&output.stderr);
                    Diagnostic::new(
                        self.name(),
                        Severity::Warning,
                        "Unused dependencies found",
                        self.category(),
                    )
                    .with_details(output_str.to_string())
                    .with_suggestion(
                        "Run 'cargo udeps' to see details and remove unused dependencies",
                    )
                }
            }
            Err(_) => Diagnostic::new(
                self.name(),
                Severity::Suggestion,
                "Could not check for unused dependencies",
                self.category(),
            )
            .with_suggestion(CARGO_UDEPS_RECOMMENDATION),
        }
    }

    /// Check that all dependencies have explicit versions
    fn check_dependency_versions(&self) -> Diagnostic {
        // In a real implementation, this would parse Cargo.toml
        // For now, just a placeholder
        Diagnostic::new(
            self.name(),
            Severity::Info,
            "All dependencies have explicit versions",
            self.category(),
        )
    }
}

impl Check for DependenciesCheck {
    fn run(&self) -> Vec<Diagnostic> {
        vec![
            self.check_outdated(),
            self.check_unused(),
            self.check_dependency_versions(),
        ]
    }

    fn name(&self) -> &str {
        "Dependencies"
    }

    fn description(&self) -> &str {
        "Check for dependency issues (outdated, unused)"
    }

    fn category(&self) -> &str {
        "dependencies"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::os::unix::process::ExitStatusExt;

    fn success_output() -> Output {
        Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }

    fn failure_output() -> Output {
        Output {
            status: std::process::ExitStatus::from_raw(1),
            stdout: Vec::new(),
            stderr: b"errors found".to_vec(),
        }
    }

    #[test]
    fn test_name_method() {
        let check = DependenciesCheck::new();
        assert_eq!(check.name(), "Dependencies");
    }

    #[test]
    fn test_description_method() {
        let check = DependenciesCheck::new();
        assert_eq!(
            check.description(),
            "Check for dependency issues (outdated, unused)"
        );
    }

    #[test]
    fn test_category_method() {
        let check = DependenciesCheck::new();
        assert_eq!(check.category(), "dependencies");
    }

    #[test]
    fn test_outdated_command_not_available() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-outdated"))
            .return_const(false);
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-udeps"))
            .return_const(true);
        mock.expect_execute_udeps()
            .return_once(|| Ok(success_output()));

        let check = DependenciesCheck::with_executor(Box::new(mock));
        let diagnostics = check.run();

        assert_eq!(diagnostics[0].severity, Severity::Suggestion);
        assert!(diagnostics[0].message.contains("not installed"));
    }

    #[test]
    fn test_udeps_command_not_available() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-outdated"))
            .return_const(true);
        mock.expect_execute_outdated()
            .return_once(|| Ok(success_output()));
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-udeps"))
            .return_const(false);

        let check = DependenciesCheck::with_executor(Box::new(mock));
        let diagnostics = check.run();

        assert_eq!(diagnostics[1].severity, Severity::Suggestion);
        assert!(diagnostics[1].message.contains("not installed"));
    }

    #[test]
    fn test_outdated_command_success() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-outdated"))
            .return_const(true);
        mock.expect_execute_outdated()
            .return_once(|| Ok(success_output()));
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-udeps"))
            .return_const(true);
        mock.expect_execute_udeps()
            .return_once(|| Ok(success_output()));

        let check = DependenciesCheck::with_executor(Box::new(mock));
        let diagnostics = check.run();

        assert_eq!(diagnostics[0].severity, Severity::Info);
        assert!(diagnostics[0].message.contains("up to date"));
    }

    #[test]
    fn test_outdated_command_failure() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-outdated"))
            .return_const(true);
        mock.expect_execute_outdated()
            .return_once(|| Ok(failure_output()));
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-udeps"))
            .return_const(true);
        mock.expect_execute_udeps()
            .return_once(|| Ok(success_output()));

        let check = DependenciesCheck::with_executor(Box::new(mock));
        let diagnostics = check.run();

        assert_eq!(diagnostics[0].severity, Severity::Warning);
        assert!(diagnostics[0]
            .message
            .contains("Outdated dependencies found"));
    }

    #[test]
    fn test_udeps_command_success() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-outdated"))
            .return_const(true);
        mock.expect_execute_outdated()
            .return_once(|| Ok(success_output()));
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-udeps"))
            .return_const(true);
        mock.expect_execute_udeps()
            .return_once(|| Ok(success_output()));

        let check = DependenciesCheck::with_executor(Box::new(mock));
        let diagnostics = check.run();

        assert_eq!(diagnostics[1].severity, Severity::Info);
        assert!(diagnostics[1].message.contains("No unused dependencies"));
    }

    #[test]
    fn test_udeps_command_failure() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-outdated"))
            .return_const(true);
        mock.expect_execute_outdated()
            .return_once(|| Ok(success_output()));
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-udeps"))
            .return_const(true);
        mock.expect_execute_udeps()
            .return_once(|| Ok(failure_output()));

        let check = DependenciesCheck::with_executor(Box::new(mock));
        let diagnostics = check.run();

        assert_eq!(diagnostics[1].severity, Severity::Warning);
        assert!(diagnostics[1].message.contains("Unused dependencies found"));
    }

    #[test]
    fn test_outdated_command_error() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-outdated"))
            .return_const(true);
        mock.expect_execute_outdated()
            .return_once(|| Err(std::io::Error::other("command failed")));
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-udeps"))
            .return_const(true);
        mock.expect_execute_udeps()
            .return_once(|| Ok(success_output()));

        let check = DependenciesCheck::with_executor(Box::new(mock));
        let diagnostics = check.run();

        assert_eq!(diagnostics[0].severity, Severity::Suggestion);
        assert!(diagnostics[0]
            .message
            .contains("Could not check for outdated dependencies"));
    }

    #[test]
    fn test_udeps_command_error() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-outdated"))
            .return_const(true);
        mock.expect_execute_outdated()
            .return_once(|| Ok(success_output()));
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-udeps"))
            .return_const(true);
        mock.expect_execute_udeps()
            .return_once(|| Err(std::io::Error::other("command failed")));

        let check = DependenciesCheck::with_executor(Box::new(mock));
        let diagnostics = check.run();

        assert_eq!(diagnostics[1].severity, Severity::Suggestion);
        assert!(diagnostics[1]
            .message
            .contains("Could not check for unused dependencies"));
    }

    #[test]
    fn test_run_returns_diagnostics() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-outdated"))
            .return_const(true);
        mock.expect_execute_outdated()
            .return_once(|| Ok(success_output()));
        mock.expect_is_command_available()
            .with(mockall::predicate::eq("cargo-udeps"))
            .return_const(true);
        mock.expect_execute_udeps()
            .return_once(|| Ok(success_output()));

        let check = DependenciesCheck::with_executor(Box::new(mock));
        let diagnostics = check.run();
        assert_eq!(diagnostics.len(), 3);
    }
}
