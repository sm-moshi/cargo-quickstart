//! Template variables implementation
//!
//! This module defines the variables available for template substitution
//! and provides functionality to build a variables object from project configuration.

use chrono::{Datelike, Local};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ProjectConfig;
use crate::ProjectType;

/// Variables available for template substitution
#[derive(Debug, Clone, Serialize)]
pub struct TemplateVariables {
    /// Project name
    pub name: String,

    /// Project description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Project version
    pub version: String,

    /// Rust edition
    pub edition: String,

    /// Author information
    pub author: Author,

    /// License type
    pub license: String,

    /// Project type flags
    pub project: ProjectFlags,

    /// Git configuration
    pub git: GitConfig,

    /// Date and timestamp information
    pub date: DateInfo,

    /// Template variant flags
    pub template: TemplateFlags,
}

/// Information about the project author
#[derive(Debug, Clone, Serialize)]
pub struct Author {
    /// Author name
    pub name: String,

    /// Author email (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Project type flags for conditional template sections
#[derive(Debug, Clone, Serialize)]
pub struct ProjectFlags {
    /// Whether the project is a binary application
    pub is_binary: bool,

    /// Whether the project is a library crate
    pub is_library: bool,
}

/// Git configuration flags
#[derive(Debug, Clone, Serialize)]
pub struct GitConfig {
    /// Whether to initialize a Git repository
    pub initialize: bool,

    /// Whether to create an initial commit
    pub create_commit: bool,
}

/// Date and timestamp information
#[derive(Debug, Clone, Serialize)]
pub struct DateInfo {
    /// Current year (YYYY)
    pub year: u32,

    /// Current date in ISO format (YYYY-MM-DD)
    pub iso_date: String,

    /// Unix timestamp
    pub timestamp: u64,
}

/// Template variant flags
#[derive(Debug, Clone, Serialize)]
pub struct TemplateFlags {
    /// Whether to use minimal templates
    pub is_minimal: bool,

    /// Whether to use extended templates
    pub is_extended: bool,
}

impl TemplateVariables {
    /// Create a new set of template variables from project configuration
    pub fn from_config(config: &ProjectConfig) -> Self {
        // Get current year and date information
        let now = Local::now();
        let year = now.year() as u32;
        let iso_date = now.format("%Y-%m-%d").to_string();

        // Generate timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Create project type flags
        let is_binary = matches!(config.project_type, ProjectType::Binary);
        let is_library = matches!(config.project_type, ProjectType::Library);

        // Default to extended templates
        let is_extended = true;
        let is_minimal = !is_extended;

        // TODO: Get author information from Git config or environment
        let author_name = "Your Name".to_string();
        let author_email = Some("your.email@example.com".to_string());

        Self {
            name: config.name.clone(),
            description: None, // TODO: Add description to config or prompt for it
            version: "0.1.0".to_string(),
            edition: config.edition.clone(),
            author: Author {
                name: author_name,
                email: author_email,
            },
            license: config.license.clone(),
            project: ProjectFlags {
                is_binary,
                is_library,
            },
            git: GitConfig {
                initialize: config.git,
                create_commit: config.git,
            },
            date: DateInfo {
                year,
                iso_date,
                timestamp,
            },
            template: TemplateFlags {
                is_minimal,
                is_extended,
            },
        }
    }

    /// Create a new set of template variables with default values for testing
    #[cfg(test)]
    pub fn default_test_variables() -> Self {
        // Skip getting actual time under Miri to avoid isolation issues
        let (year, iso_date) = if cfg!(miri) {
            (2023, "2023-04-01".to_string())
        } else {
            let now = Local::now();
            (now.year() as u32, now.format("%Y-%m-%d").to_string())
        };

        Self {
            name: "test-project".to_string(),
            description: Some("A test project".to_string()),
            version: "0.1.0".to_string(),
            edition: "2021".to_string(),
            author: Author {
                name: "Test Author".to_string(),
                email: Some("test@example.com".to_string()),
            },
            license: "MIT".to_string(),
            project: ProjectFlags {
                is_binary: true,
                is_library: false,
            },
            git: GitConfig {
                initialize: true,
                create_commit: true,
            },
            date: DateInfo {
                year,
                iso_date,
                timestamp: 1619712000, // Fixed timestamp for testing
            },
            template: TemplateFlags {
                is_minimal: false,
                is_extended: true,
            },
        }
    }
}
