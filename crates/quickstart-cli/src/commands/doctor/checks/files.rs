//! Check for required project files

use crate::commands::doctor::diagnosis::Check;
use crate::commands::doctor::types::{Diagnostic, Severity};
use std::path::Path;

/// A check for required project files
pub struct FilesCheck {
    dir: Box<Path>,
}

impl FilesCheck {
    /// Create a new FilesCheck for the given directory
    pub fn new(dir: &Path) -> Self {
        Self { dir: dir.into() }
    }
}

impl Check for FilesCheck {
    fn run(&self) -> Vec<Diagnostic> {
        let files = [
            ("Cargo.toml", "Project manifest (Cargo.toml) is required."),
            ("README.md", "README.md is recommended for documentation."),
            (
                ".gitignore",
                ".gitignore is recommended to avoid committing build artifacts.",
            ),
            (
                "LICENSE",
                "A LICENSE file is recommended for open source projects.",
            ),
        ];

        files
            .iter()
            .map(|(file, msg)| {
                let exists = self.dir.join(file).exists();

                if exists {
                    Diagnostic::new(
                        self.name(),
                        Severity::Info,
                        format!("{file} found."),
                        "files",
                    )
                } else if *file == "Cargo.toml" {
                    Diagnostic::new(
                        self.name(),
                        Severity::Error,
                        format!("{file} is missing."),
                        "files",
                    )
                    .with_suggestion(msg.to_string())
                } else {
                    Diagnostic::new(
                        self.name(),
                        Severity::Warning,
                        format!("{file} is missing."),
                        "files",
                    )
                    .with_suggestion(msg.to_string())
                }
            })
            .collect()
    }

    fn name(&self) -> &str {
        "Required Files"
    }

    fn description(&self) -> &str {
        "Check for required and recommended project files"
    }

    fn category(&self) -> &str {
        "files"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs;
    use std::io::Result as IoResult;
    use tempfile::{tempdir, TempDir};

    // Helper function to create test dir with proper error handling
    fn setup_test_dir(files: &[&str]) -> IoResult<TempDir> {
        // Skip filesystem operations under Miri
        if cfg!(miri) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Skipping file system tests under Miri",
            ));
        }

        let dir = tempdir()?;
        for file in files {
            fs::write(dir.path().join(file), "test")?;
        }
        Ok(dir)
    }

    #[test]
    fn test_all_files_present() -> IoResult<()> {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return Ok(());
        }

        let files = ["Cargo.toml", "README.md", ".gitignore", "LICENSE"];
        let dir = setup_test_dir(&files)?;

        let check = FilesCheck::new(dir.path());
        let diagnostics = check.run();

        assert_eq!(diagnostics.len(), 4);
        assert!(diagnostics.iter().all(|d| d.severity == Severity::Info));
        Ok(())
    }

    #[test]
    fn test_all_files_missing() -> IoResult<()> {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return Ok(());
        }

        let dir = tempdir()?;

        let check = FilesCheck::new(dir.path());
        let diagnostics = check.run();

        assert_eq!(diagnostics.len(), 4);
        assert_eq!(diagnostics[0].severity, Severity::Error); // Cargo.toml
        assert!(diagnostics[1..]
            .iter()
            .all(|d| d.severity == Severity::Warning));
        Ok(())
    }

    #[test]
    fn test_some_files_missing() -> IoResult<()> {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return Ok(());
        }

        let dir = tempdir()?;
        fs::write(dir.path().join("Cargo.toml"), "test")?;
        fs::write(dir.path().join("README.md"), "test")?;

        let check = FilesCheck::new(dir.path());
        let diagnostics = check.run();

        assert_eq!(diagnostics.len(), 4);

        // Check severities
        let cargo_toml = diagnostics
            .iter()
            .find(|d| d.message.contains("Cargo.toml"))
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Cargo.toml diagnostic not found",
                )
            })?;
        let readme = diagnostics
            .iter()
            .find(|d| d.message.contains("README.md"))
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "README.md diagnostic not found",
                )
            })?;
        let gitignore = diagnostics
            .iter()
            .find(|d| d.message.contains(".gitignore"))
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    ".gitignore diagnostic not found",
                )
            })?;
        let license = diagnostics
            .iter()
            .find(|d| d.message.contains("LICENSE"))
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotFound, "LICENSE diagnostic not found")
            })?;

        assert_eq!(cargo_toml.severity, Severity::Info);
        assert_eq!(readme.severity, Severity::Info);
        assert_eq!(gitignore.severity, Severity::Warning);
        assert_eq!(license.severity, Severity::Warning);
        Ok(())
    }
}
