//! Check for Rust toolchain

use crate::commands::doctor::diagnosis::Check;
use crate::commands::doctor::types::{Diagnostic, Severity};
use std::path::{Path, PathBuf};
use std::process::Output;

pub type WhichExecutor = fn(&str) -> std::io::Result<PathBuf>;
pub type CommandExecutor = fn(&Path, &str) -> std::io::Result<Output>;

/// Check for Rust toolchain
pub struct RustToolchainCheck {
    which_executor: WhichExecutor,
    command_executor: CommandExecutor,
}

impl RustToolchainCheck {
    /// Create a new RustToolchainCheck with default executors
    pub fn new() -> Self {
        Self {
            which_executor: which_fn,
            command_executor: run_version_fn,
        }
    }

    /// Create a RustToolchainCheck with custom executors (useful for testing)
    #[allow(dead_code)]
    pub fn with_executors(
        which_executor: WhichExecutor,
        command_executor: CommandExecutor,
    ) -> Self {
        Self {
            which_executor,
            command_executor,
        }
    }

    /// Check for a specific tool (cargo or rustc)
    fn check_tool(&self, tool: &str) -> Diagnostic {
        match (self.which_executor)(tool) {
            Ok(path) => {
                let version_result: std::io::Result<Output> =
                    (self.command_executor)(&path, "--version");
                match version_result {
                    Ok(output) => {
                        if output.status.success() {
                            Diagnostic::new(
                                self.name(),
                                Severity::Info,
                                format!(
                                    "{tool}: {}",
                                    String::from_utf8_lossy(&output.stdout).trim()
                                ),
                                self.category(),
                            )
                        } else {
                            Diagnostic::new(
                                self.name(),
                                Severity::Warning,
                                format!("{tool} found, but failed to run '{tool} --version'"),
                                self.category(),
                            )
                            .with_suggestion(format!("Check your {tool} installation."))
                        }
                    }
                    Err(_) => Diagnostic::new(
                        self.name(),
                        Severity::Warning,
                        format!("{tool} found, but could not execute '{tool} --version'"),
                        self.category(),
                    )
                    .with_suggestion(format!("Check your {tool} installation.")),
                }
            }
            Err(_) => Diagnostic::new(
                self.name(),
                Severity::Error,
                format!("{tool} not found in $PATH"),
                self.category(),
            )
            .with_suggestion(format!("Install Rust and {tool} from https://rustup.rs/")),
        }
    }
}

impl Check for RustToolchainCheck {
    fn run(&self) -> Vec<Diagnostic> {
        let diagnostics = vec![self.check_tool("cargo"), self.check_tool("rustc")];

        diagnostics
    }

    fn name(&self) -> &str {
        "Rust Toolchain"
    }

    fn description(&self) -> &str {
        "Check for presence and basic functionality of Rust toolchain (cargo, rustc)"
    }

    fn category(&self) -> &str {
        "rust"
    }
}

/// Production version of the which executor using which::which
fn which_fn(path: &str) -> std::io::Result<PathBuf> {
    which::which(path).map_err(std::io::Error::other)
}

/// Production version of the command executor using std::process::Command
fn run_version_fn(path: &Path, arg: &str) -> std::io::Result<Output> {
    std::process::Command::new(path).arg(arg).output()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::process::ExitStatusExt;

    fn mock_output(success: bool, stdout: &str) -> Output {
        Output {
            status: if success {
                std::process::ExitStatus::from_raw(0)
            } else {
                std::process::ExitStatus::from_raw(1)
            },
            stdout: stdout.as_bytes().to_vec(),
            stderr: vec![],
        }
    }

    fn mock_path(tool: &str) -> PathBuf {
        PathBuf::from(format!("/mock/{tool}"))
    }

    #[test]
    fn test_both_tools_ok() {
        fn which_executor(tool: &str) -> std::io::Result<PathBuf> {
            Ok(mock_path(tool))
        }
        fn command_executor(_: &Path, _: &str) -> std::io::Result<Output> {
            Ok(mock_output(true, "version 1.0"))
        }

        let check = RustToolchainCheck::with_executors(which_executor, command_executor);
        let diagnostics = check.run();

        assert_eq!(diagnostics.len(), 2);
        assert_eq!(diagnostics[0].severity, Severity::Info);
        assert_eq!(diagnostics[1].severity, Severity::Info);
    }

    #[test]
    fn test_cargo_missing() {
        fn which_executor(tool: &str) -> std::io::Result<PathBuf> {
            if tool == "cargo" {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "not found",
                ))
            } else {
                Ok(mock_path(tool))
            }
        }
        fn command_executor(_: &Path, _: &str) -> std::io::Result<Output> {
            Ok(mock_output(true, "version 1.0"))
        }

        let check = RustToolchainCheck::with_executors(which_executor, command_executor);
        let diagnostics = check.run();

        assert_eq!(diagnostics[0].severity, Severity::Error);
        assert_eq!(diagnostics[1].severity, Severity::Info);
    }

    #[test]
    fn test_rustc_missing() {
        fn which_executor(tool: &str) -> std::io::Result<PathBuf> {
            if tool == "rustc" {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "not found",
                ))
            } else {
                Ok(mock_path(tool))
            }
        }
        fn command_executor(_: &Path, _: &str) -> std::io::Result<Output> {
            Ok(mock_output(true, "version 1.0"))
        }

        let check = RustToolchainCheck::with_executors(which_executor, command_executor);
        let diagnostics = check.run();

        assert_eq!(diagnostics[0].severity, Severity::Info);
        assert_eq!(diagnostics[1].severity, Severity::Error);
    }

    #[test]
    fn test_cargo_version_fails() {
        fn which_executor(tool: &str) -> std::io::Result<PathBuf> {
            Ok(mock_path(tool))
        }
        fn command_executor(path: &Path, _: &str) -> std::io::Result<Output> {
            if path.ends_with("cargo") {
                Ok(mock_output(false, ""))
            } else {
                Ok(mock_output(true, "version 1.0"))
            }
        }

        let check = RustToolchainCheck::with_executors(which_executor, command_executor);
        let diagnostics = check.run();

        assert_eq!(diagnostics[0].severity, Severity::Warning);
        assert_eq!(diagnostics[1].severity, Severity::Info);
    }
}
