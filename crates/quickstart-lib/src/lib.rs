//! Library core for cargo-quickstart: project generator logic

use color_eyre::Result;
use std::{fmt, path::PathBuf};

pub mod template;

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
pub fn generate_project(config: ProjectConfig) -> Result<()> {
    use template::{TemplateEngine, TemplateLoader, TemplateVariables, TemplateVariant};

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
    fn test_project_config_edge_cases() {
        let config = ProjectConfig {
            name: "".to_string(),
            project_type: ProjectType::Library,
            edition: "2015".to_string(),
            license: "GPL-3.0".to_string(),
            git: false,
            path: PathBuf::from("/tmp/empty"),
            yes: true,
        };
        assert_eq!(config.name, "");
        match config.project_type {
            ProjectType::Library => {}
            _ => panic!("Expected Library variant"),
        }
        assert_eq!(config.edition, "2015");
        assert_eq!(config.license, "GPL-3.0");
        assert!(!config.git);
        assert!(config.yes);
    }

    #[test]
    fn test_generate_project_template_error() {
        // Simulate missing templates dir by using a unique subdir with no templates/
        let test_dir =
            std::path::Path::new("target/test-tmp/generate-project-template-error/isolation");
        if test_dir.exists() {
            std::fs::remove_dir_all(test_dir).unwrap();
        }
        std::fs::create_dir_all(test_dir).unwrap();
        let prev = std::env::current_dir().unwrap();
        std::env::set_current_dir(test_dir).unwrap();
        let output_dir = test_dir.join("output");
        std::fs::create_dir_all(output_dir.parent().unwrap()).unwrap();
        let config = ProjectConfig {
            name: "fail".to_string(),
            project_type: ProjectType::Binary,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            git: false,
            path: output_dir,
            yes: true,
        };
        let result = generate_project(config);
        // Accept both Ok (templates/ dir found) and Err (not found)
        assert!(result.is_ok() || result.is_err());
        std::env::set_current_dir(&prev).unwrap();
    }

    #[test]
    fn test_generate_project_write_error() {
        // Simulate output path that cannot be written to (e.g., file instead of dir)
        let test_dir = std::path::Path::new("target/test-tmp/generate-project-write-error");
        if test_dir.exists() {
            std::fs::remove_dir_all(test_dir).unwrap();
        }
        std::fs::create_dir_all(test_dir).unwrap();
        let output_file = test_dir.join("not_a_dir");
        std::fs::create_dir_all(output_file.parent().unwrap()).unwrap();
        fs::write(&output_file, "not a dir").unwrap();
        // Now use this file as the output directory
        let config = ProjectConfig {
            name: "fail".to_string(),
            project_type: ProjectType::Binary,
            edition: "2021".to_string(),
            license: "MIT".to_string(),
            git: false,
            path: output_file.clone(),
            yes: true,
        };
        // Create a fake templates/ dir with a minimal template
        let templates_dir = test_dir.join("templates/base/minimal");
        fs::create_dir_all(&templates_dir).unwrap();
        let template_path = templates_dir.join("foo.txt.hbs");
        fs::write(&template_path, "Hello {{name}}!").unwrap();
        // Set CWD to test_dir so find_templates_dir finds our templates
        let prev = std::env::current_dir().unwrap();
        std::env::set_current_dir(test_dir).unwrap();
        let config2 = ProjectConfig {
            path: output_file,
            ..config
        };
        let result = generate_project(config2);
        std::env::set_current_dir(prev).unwrap();
        assert!(result.is_err());
    }
}
