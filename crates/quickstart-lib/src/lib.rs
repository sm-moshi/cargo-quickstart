//! Library core for cargo-quickstart: project generator logic

use color_eyre::Result;
use std::{fmt, path::PathBuf};

/// Project type (binary or library)
#[derive(Debug, Clone, Copy)]
pub enum ProjectType {
    /// A binary application
    Binary,
    /// A library crate
    Library,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::Binary => write!(f, "Binary application"),
            ProjectType::Library => write!(f, "Library crate"),
        }
    }
}

/// Configuration for scaffolding a new project
#[derive(Debug)]
pub struct ProjectConfig {
    /// Project name
    pub name: String,
    /// Project type (binary or library)
    pub project_type: ProjectType,
    /// Rust edition
    pub edition: String,
    /// License
    pub license: String,
    /// Initialize git repository
    pub git: bool,
    /// Target path
    pub path: PathBuf,
    /// Accept all defaults without prompting
    pub yes: bool,
}

/// Generate a new project based on the provided configuration
pub fn generate_project(config: ProjectConfig) -> Result<()> {
    // TODO: Implement scaffolding logic here with the following steps:
    // 1. Create project directory structure
    // 2. Render and write template files
    // 3. Initialize Git repository if requested
    // 4. Install development tools if needed

    println!("Generating project: {config:#?}");
    Ok(())
}

/// Config type for backward compatibility
#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub bin: bool,
    pub lib: bool,
    pub edition: String,
    pub license: String,
    pub git: bool,
    pub path: PathBuf,
    pub yes: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_project_type_display() {
        assert_eq!(ProjectType::Binary.to_string(), "Binary application");
        assert_eq!(ProjectType::Library.to_string(), "Library crate");
    }
}
