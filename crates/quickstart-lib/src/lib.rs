#![cfg_attr(test, allow(clippy::disallowed_methods))]

//! Library core for cargo-quickstart: project generator logic

use crate::config::QuickstartConfig;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf};

pub mod config;
pub mod template;
pub mod tools;

/// Project type (binary or library)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

/// Find the nearest `templates/` directory by walking up from the current directory.
pub fn find_templates_dir() -> Result<PathBuf, std::io::Error> {
    let mut dir = std::env::current_dir()?;
    loop {
        let candidate = dir.join("templates");
        if candidate.is_dir() {
            return Ok(candidate);
        }
        if !dir.pop() {
            break;
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Could not find a 'templates/' directory in this or any parent directory.",
    ))
}

/// Generate a new project based on the provided configuration
pub fn generate_project<C: Into<QuickstartConfig>>(config: C) -> Result<()> {
    use template::{TemplateEngine, TemplateLoader, TemplateVariables, TemplateVariant};

    let config: QuickstartConfig = config.into();
    // Validate that the parent directory exists
    if let Some(parent) = config.path.parent() {
        if !parent.exists() {
            return Err(color_eyre::eyre::eyre!(
                "Parent directory '{}' does not exist. Please create it first.",
                parent.display()
            ));
        }
    }

    // Initialize template variables from config
    let variables = TemplateVariables::from_config(&config);

    // Create the template engine
    let engine = TemplateEngine::new(variables);

    // Smarter template path resolution: search upwards for templates/
    let template_path = find_templates_dir()?;
    let loader = TemplateLoader::new(template_path);

    // Use extended template variant by default
    let variant = TemplateVariant::Extended;

    // List all templates for this project type
    let templates = loader.list_templates(config.project_type, variant)?;

    // Create the output directory
    std::fs::create_dir_all(&config.path)?;

    // Process each template
    for template_path in templates {
        // Get relative path for loading template
        let rel_path = pathdiff::diff_paths(&template_path, loader.base_path())
            .unwrap_or_else(|| template_path.clone());
        let rel_path_str = rel_path.to_string_lossy();

        // Load template content
        let template_content = loader.load_template(&rel_path_str)?;

        // Render the template
        let rendered = engine.render_template(&template_content)?;

        // Determine output path
        let output_path = loader.get_destination_path(&template_path, &config.path);

        // Create parent directories if needed
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Write rendered content to file
        std::fs::write(output_path, rendered)?;
    }

    // TODO: Initialize Git repository if requested
    if config.git {
        // git::init_repository(&config.path)?;
    }

    println!("Successfully generated project: {}", config.name);
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
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_project_type_display() {
        assert_eq!(ProjectType::Binary.to_string(), "Binary application");
        assert_eq!(ProjectType::Library.to_string(), "Library crate");
    }

    #[test]
    fn test_find_templates_dir_error() {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return;
        }

        // Use a temp dir with no templates/ parent
        let dir = tempdir().unwrap();
        let prev = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();
        let result = find_templates_dir();
        std::env::set_current_dir(prev).unwrap();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_struct_instantiation() {
        let config = Config {
            name: "foo".to_string(),
            bin: true,
            lib: false,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            git: true,
            path: PathBuf::from("/tmp/foo"),
            yes: false,
        };
        assert_eq!(config.name, "foo");
        assert!(config.bin);
    }

    #[test]
    fn test_generate_project_write_error() {
        // Skip under Miri
        if cfg!(miri) {
            eprintln!("Skipping file system test under Miri");
            return;
        }

        // Create a temporary directory for this test
        let test_dir = tempfile::tempdir().unwrap();
        let test_path = test_dir.path();

        // Create an output file (not a directory) to cause the write error
        let output_file = test_path.join("output_file");
        fs::write(&output_file, "not a dir").unwrap();

        // Create a proper templates directory structure following the expected pattern:
        // templates/
        //   ├── base/
        //   ├── binary/
        //   │   ├── minimal/
        //   │   └── extended/
        //   └── library/
        //       ├── minimal/
        //       └── extended/

        let templates_dir = test_path.join("templates");
        let base_dir = templates_dir.join("base");
        let binary_dir = templates_dir.join("binary");
        let binary_extended_dir = binary_dir.join("extended");
        let binary_minimal_dir = binary_dir.join("minimal");
        let library_dir = templates_dir.join("library");
        let library_extended_dir = library_dir.join("extended");
        let library_minimal_dir = library_dir.join("minimal");

        // Create all the required directories
        fs::create_dir_all(&base_dir).unwrap();
        fs::create_dir_all(&binary_extended_dir).unwrap();
        fs::create_dir_all(&binary_minimal_dir).unwrap();
        fs::create_dir_all(&library_extended_dir).unwrap();
        fs::create_dir_all(&library_minimal_dir).unwrap();

        // Create template files
        fs::write(
            base_dir.join("README.md.hbs"),
            "# {{name}}\n\nThis is a test project.",
        )
        .unwrap();

        fs::write(
            binary_extended_dir.join("main.rs.hbs"),
            "fn main() {\n    println!(\"Hello from {{name}}!\");\n}",
        )
        .unwrap();

        fs::write(
            binary_minimal_dir.join("main.rs.hbs"),
            "fn main() {\n    println!(\"Minimal {{name}}!\");\n}",
        )
        .unwrap();

        fs::write(
            library_extended_dir.join("lib.rs.hbs"),
            "pub fn hello() {\n    println!(\"Hello from {{name}} library!\");\n}",
        )
        .unwrap();

        fs::write(
            library_minimal_dir.join("lib.rs.hbs"),
            "pub fn hello() {}\n",
        )
        .unwrap();

        // Save current directory and change to test directory to find templates/
        let prev = std::env::current_dir().unwrap();
        std::env::set_current_dir(test_path).unwrap();

        // Create config pointing to the file (not directory) as output path
        let config = QuickstartConfig {
            name: "test-project".to_string(),
            project_type: ProjectType::Binary,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            git: false,
            path: output_file,
            yes: true,
            description: None,
            author: None,
            features: None,
            plugins: None,
            dry_run: false,
            template_variant: None,
        };

        // This should fail because the output path is a file, not a directory
        let result = generate_project(config);

        // Restore previous working directory
        std::env::set_current_dir(prev).unwrap();

        // Verify that an error occurred
        assert!(result.is_err(), "Should error when output path is a file");

        // Check that the error is related to the file operation
        if let Err(e) = result {
            assert!(
                e.to_string().contains("Not a directory")
                    || e.to_string().contains("Is a file")
                    || e.to_string().contains("already exists")
                    || e.to_string().contains("Permission denied")
                    || e.to_string().contains("File exists"),
                "Error should be about output path being a file, got: {e}"
            );
        }
    }
}
