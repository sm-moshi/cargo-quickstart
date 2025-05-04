//! Template loader implementation
//!
//! This module provides functionality for loading templates from the filesystem
//! and managing template paths.

use std::fs;
use std::path::{Path, PathBuf};

use crate::ProjectType;

use super::{Result, TemplateError, TemplateVariant};

/// Template loader for file-based templates
pub struct TemplateLoader {
    /// Base directory for templates
    base_path: PathBuf,
}

impl TemplateLoader {
    /// Create a new template loader with the given base path
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    /// Load a template from the filesystem
    pub fn load_template(&self, template_path: &str) -> Result<String> {
        let full_path = self.base_path.join(template_path);
        fs::read_to_string(&full_path).map_err(|e| TemplateError::LoadError {
            path: template_path.to_string(),
            source: e,
        })
    }

    /// Check if a template exists at the given path
    pub fn template_exists(&self, template_path: &str) -> bool {
        let full_path = self.base_path.join(template_path);
        full_path.exists()
    }

    /// List all templates applicable for a project type and variant
    pub fn list_templates(
        &self,
        project_type: ProjectType,
        variant: TemplateVariant,
    ) -> Result<Vec<PathBuf>> {
        // Build the directory path for this project type and variant
        let type_dir = match project_type {
            ProjectType::Binary => "binary",
            ProjectType::Library => "library",
        };

        let variant_dir = match variant {
            TemplateVariant::Minimal => "minimal",
            TemplateVariant::Extended => "extended",
        };

        let template_dir = self.base_path.join(type_dir).join(variant_dir);
        let base_dir = self.base_path.join("base");

        println!("Template directory: {}", template_dir.display());
        println!("Base directory: {}", base_dir.display());

        // Return error if template directory doesn't exist
        if !template_dir.exists() {
            println!("Template directory does not exist!");
            return Err(TemplateError::TemplateNotFound {
                path: template_dir.to_string_lossy().to_string(),
            });
        }

        // Collect templates from base directory if it exists
        let mut templates = if base_dir.exists() {
            println!("Base directory exists, collecting templates...");
            self.collect_templates_from_dir(&base_dir)?
        } else {
            println!("Base directory does not exist!");
            Vec::new()
        };

        // Collect templates from project type directory
        println!("Collecting templates from project type directory...");
        let type_templates = self.collect_templates_from_dir(&template_dir)?;
        templates.extend(type_templates);

        println!("Found {} templates", templates.len());
        for template in &templates {
            println!("  - {}", template.display());
        }

        Ok(templates)
    }

    /// Recursively collect templates from a directory
    #[allow(clippy::only_used_in_recursion)]
    fn collect_templates_from_dir(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        if !dir.exists() {
            return Err(TemplateError::TemplateNotFound {
                path: dir.to_string_lossy().to_string(),
            });
        }

        let mut templates = Vec::new();

        for entry in fs::read_dir(dir).map_err(|e| TemplateError::LoadError {
            path: dir.to_string_lossy().to_string(),
            source: e,
        })? {
            let entry = entry.map_err(|e| TemplateError::LoadError {
                path: dir.to_string_lossy().to_string(),
                source: e,
            })?;

            let path = entry.path();

            if path.is_dir() {
                // Recursively collect templates from subdirectories
                let sub_templates = self.collect_templates_from_dir(&path)?;
                templates.extend(sub_templates);
            } else {
                // Only include files with .hbs extension
                if let Some(ext) = path.extension() {
                    if ext == "hbs" {
                        templates.push(path);
                    }
                }
            }
        }

        Ok(templates)
    }

    /// Get the destination path for a template
    pub fn get_destination_path(&self, template_path: &Path, dest_root: &Path) -> PathBuf {
        // Calculate the relative path from base_path to template_path
        let rel_path = pathdiff::diff_paths(template_path, &self.base_path)
            .unwrap_or_else(|| template_path.to_path_buf());

        // If the template is from the 'base/' directory, strip 'base/' from the path
        let rel_path = rel_path
            .strip_prefix("base")
            .unwrap_or(&rel_path)
            .to_path_buf();

        // If the template is from a project type directory, strip the project type and variant
        let rel_path = if let Some(components) = rel_path.to_str() {
            let components: Vec<&str> = components.split('/').collect();
            if components.first() == Some(&"library") || components.first() == Some(&"binary") {
                if components.len() >= 3 {
                    // Skip project type and variant directories
                    let remaining = components[2..].join("/");
                    PathBuf::from(remaining)
                } else {
                    rel_path
                }
            } else {
                rel_path
            }
        } else {
            rel_path
        };

        // Join with destination root to get final path
        let dest_path = dest_root.join(rel_path);

        // For template files (.hbs extension), remove the extension
        if let Some(ext) = dest_path.extension() {
            if ext == "hbs" {
                return dest_path.with_extension("");
            }
        }

        dest_path
    }

    /// Get the base path of the template loader
    pub fn base_path(&self) -> &Path {
        &self.base_path
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_template_dir() -> TempDir {
        let temp_dir = tempfile::tempdir().unwrap();

        // Create base directory
        fs::create_dir_all(temp_dir.path().join("base")).unwrap();

        // Create binary directory structure
        fs::create_dir_all(temp_dir.path().join("binary/minimal/src")).unwrap();
        fs::create_dir_all(temp_dir.path().join("binary/extended/src")).unwrap();

        // Create library directory structure
        fs::create_dir_all(temp_dir.path().join("library/minimal/src")).unwrap();
        fs::create_dir_all(temp_dir.path().join("library/extended/src")).unwrap();

        // Create some test template files
        let base_readme = temp_dir.path().join("base/README.md.hbs");
        let mut file = fs::File::create(&base_readme).unwrap();
        writeln!(file, "# {{{{name}}}}\n\n{{{{description}}}}\n").unwrap();

        let binary_main = temp_dir.path().join("binary/minimal/src/main.rs.hbs");
        let mut file = fs::File::create(&binary_main).unwrap();
        writeln!(
            file,
            "fn main() {{\n    println!(\"Hello from {{name}}!\");\n}}"
        )
        .unwrap();

        let library_lib = temp_dir.path().join("library/minimal/src/lib.rs.hbs");
        let mut file = fs::File::create(&library_lib).unwrap();
        writeln!(
            file,
            "//! {{name}} library\n\npub fn add(a: i32, b: i32) -> i32 {{\n    a + b\n}}"
        )
        .unwrap();

        temp_dir
    }

    #[test]
    fn test_load_template() {
        let temp_dir = create_test_template_dir();
        let loader = TemplateLoader::new(temp_dir.path());

        let template_content = loader.load_template("base/README.md.hbs").unwrap();
        assert!(template_content.contains("# {{name}}"));
    }

    #[test]
    fn test_template_exists() {
        let temp_dir = create_test_template_dir();
        let loader = TemplateLoader::new(temp_dir.path());

        assert!(loader.template_exists("base/README.md.hbs"));
        assert!(!loader.template_exists("nonexistent.hbs"));
    }

    #[test]
    fn test_list_templates() {
        let temp_dir = create_test_template_dir();
        let loader = TemplateLoader::new(temp_dir.path());

        let templates = loader
            .list_templates(ProjectType::Binary, TemplateVariant::Minimal)
            .unwrap();

        // Should find at least the base README and binary main.rs
        assert!(templates.len() >= 2);

        // Check that the expected templates are included
        let has_readme = templates
            .iter()
            .any(|path| path.to_string_lossy().contains("README.md.hbs"));
        let has_main = templates
            .iter()
            .any(|path| path.to_string_lossy().contains("main.rs.hbs"));

        assert!(has_readme);
        assert!(has_main);
    }

    #[test]
    fn test_get_destination_path() {
        let temp_dir = create_test_template_dir();
        let loader = TemplateLoader::new(temp_dir.path());

        let template_path = temp_dir.path().join("base/README.md.hbs");
        let dest_root = PathBuf::from("/tmp/my-project");

        let dest_path = loader.get_destination_path(&template_path, &dest_root);

        // Should be /tmp/my-project/README.md (without .hbs extension)
        assert_eq!(dest_path, PathBuf::from("/tmp/my-project/README.md"));
    }
}
