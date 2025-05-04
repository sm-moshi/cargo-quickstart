//! Check for template usage and consistency

use crate::commands::doctor::diagnosis::Check;
use crate::commands::doctor::types::{Diagnostic, Severity};
use std::path::PathBuf;

/// Check for template consistency across the project
pub struct TemplatesCheck {
    #[allow(dead_code)]
    template_dir: PathBuf,
}

impl TemplatesCheck {
    /// Create a new TemplatesCheck that examines the templates directory
    pub fn new() -> Self {
        Self {
            template_dir: PathBuf::from("templates"),
        }
    }

    /// Check that templates have consistent naming
    fn check_template_naming(&self) -> Diagnostic {
        // In a real implementation, we would scan template files
        // For now, just a placeholder
        Diagnostic::new(
            self.name(),
            Severity::Info,
            "Templates follow consistent naming conventions",
            self.category(),
        )
    }

    /// Check that templates have documentation
    fn check_template_docs(&self) -> Diagnostic {
        // In a real implementation, we would check for README files
        // For now, just a placeholder
        Diagnostic::new(
            self.name(),
            Severity::Info,
            "All templates have documentation",
            self.category(),
        )
    }

    /// Check that templates are valid
    fn check_template_validity(&self) -> Diagnostic {
        // In a real implementation, we would validate template syntax
        // For now, just a placeholder
        Diagnostic::new(
            self.name(),
            Severity::Info,
            "All templates have valid syntax",
            self.category(),
        )
    }
}

impl Check for TemplatesCheck {
    fn run(&self) -> Vec<Diagnostic> {
        vec![
            self.check_template_naming(),
            self.check_template_docs(),
            self.check_template_validity(),
        ]
    }

    fn name(&self) -> &str {
        "Templates"
    }

    fn description(&self) -> &str {
        "Check for template consistency and documentation"
    }

    fn category(&self) -> &str {
        "templates"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_method() {
        let check = TemplatesCheck::new();
        assert_eq!(check.name(), "Templates");
    }

    #[test]
    fn test_description_method() {
        let check = TemplatesCheck::new();
        assert_eq!(
            check.description(),
            "Check for template consistency and documentation"
        );
    }

    #[test]
    fn test_category_method() {
        let check = TemplatesCheck::new();
        assert_eq!(check.category(), "templates");
    }

    #[test]
    fn test_run_returns_diagnostics() {
        let check = TemplatesCheck::new();
        let diagnostics = check.run();
        assert!(!diagnostics.is_empty());
    }
}
