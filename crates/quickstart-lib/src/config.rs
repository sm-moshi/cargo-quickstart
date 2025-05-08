//! Config handling for cargo-quickstart library
//!
//! This module provides the config handling for the cargo-quickstart library.
//!
//! The config handling is currently under development and is not yet available.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::ProjectType;

/// Author information for the project
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Author {
    /// Author's full name
    pub name: String,
    /// Author's email address (optional)
    pub email: Option<String>,
}

/// Template variant selection for project scaffolding
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TemplateVariant {
    /// Minimal template set
    Minimal,
    /// Extended template set (default)
    #[default]
    Extended,
}

/// Canonical configuration for project scaffolding in cargo-quickstart
///
/// This struct is used by all UX modes (wizard, manual, TUI) to drive project generation.
/// It is designed to be extensible and forward-compatible for future features.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickstartConfig {
    /// Name of the project (valid Rust crate name)
    pub name: String,
    /// Project type (binary or library)
    pub project_type: ProjectType,
    /// Rust edition (e.g., "2021")
    pub edition: String,
    /// License identifier (e.g., "MIT", "Apache-2.0")
    pub license: String,
    /// Whether to initialise a Git repository
    pub git: bool,
    /// Target path for project generation
    pub path: PathBuf,
    /// Accept all defaults without prompting
    pub yes: bool,
    /// Optional project description
    pub description: Option<String>,
    /// Author information (optional)
    pub author: Option<Author>,
    /// Optional list of features to include (e.g., README, CI, VSCode)
    pub features: Option<Vec<String>>,
    /// Optional list of plugins to enable
    pub plugins: Option<Vec<String>>,
    /// Enable dry-run mode (preview changes without writing to disk)
    pub dry_run: bool,
    /// Template variant to use (minimal or extended)
    pub template_variant: Option<TemplateVariant>,
}

impl Default for QuickstartConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            project_type: ProjectType::Binary,
            edition: "2021".to_string(),
            license: "MIT OR Apache-2.0".to_string(),
            git: false,
            path: PathBuf::new(),
            yes: false,
            description: None,
            author: None,
            features: None,
            plugins: None,
            dry_run: false,
            template_variant: Some(TemplateVariant::Extended),
        }
    }
}
