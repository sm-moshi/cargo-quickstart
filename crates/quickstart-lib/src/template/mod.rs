//! Template system for project generation
//!
//! This module handles loading, rendering, and managing templates for
//! project scaffolding with variable substitution and conditional sections.

mod engine;
mod loader;
mod variables;

pub use engine::TemplateEngine;
pub use loader::TemplateLoader;
pub use variables::TemplateVariables;

/// Represents the variant of templates to use
#[derive(Debug, Clone, Copy)]
pub enum TemplateVariant {
    /// Essential files only
    Minimal,
    /// Full-featured setup with additional tooling
    Extended,
}

impl Default for TemplateVariant {
    fn default() -> Self {
        Self::Extended
    }
}

/// Error type for template-related operations
#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    /// Failed to load template from filesystem
    #[error("Failed to load template at {path}: {source}")]
    LoadError {
        /// Template path
        path: String,
        /// Source error
        source: std::io::Error,
    },

    /// Failed to render template
    #[error("Failed to render template {name}: {source}")]
    RenderError {
        /// Template name
        name: String,
        /// Source error
        source: handlebars::RenderError,
    },

    /// Template was not found
    #[error("Template not found: {path}")]
    TemplateNotFound {
        /// Template path
        path: String,
    },
}

/// Result type for template operations
pub type Result<T> = std::result::Result<T, TemplateError>;

#[cfg(test)]
mod tests {
    use super::*;
    use handlebars::RenderErrorReason;
    use pretty_assertions::assert_eq;
    use std::fmt::Debug;
    use std::io;

    #[test]
    fn test_template_variant_enum() {
        let _min = TemplateVariant::Minimal;
        let _ext = TemplateVariant::Extended;
        assert_eq!(
            TemplateVariant::default() as u8,
            TemplateVariant::Extended as u8
        );
    }

    #[test]
    fn test_template_error_variants() {
        // LoadError
        let err = TemplateError::LoadError {
            path: "foo".to_string(),
            source: io::Error::other("fail"),
        };
        assert!(format!("{err}").contains("Failed to load template"));

        // RenderError
        let render_err = TemplateError::RenderError {
            name: "bar".to_string(),
            source: handlebars::RenderError::from(RenderErrorReason::Other("fail".into())),
        };
        assert!(format!("{render_err}").contains("Failed to render template"));

        // TemplateNotFound
        let not_found = TemplateError::TemplateNotFound {
            path: "baz".to_string(),
        };
        assert!(format!("{not_found}").contains("Template not found"));
    }

    #[test]
    fn test_result_type() {
        fn takes_result<T: Debug>(r: Result<T>) -> bool {
            r.is_ok()
        }
        let ok: Result<u8> = Ok(42);
        let err: Result<u8> = Err(TemplateError::TemplateNotFound { path: "x".into() });
        assert!(takes_result(ok));
        assert!(!takes_result(err));
    }
}
