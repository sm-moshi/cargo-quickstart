//! Project diagnostic system for the doctor command
//!
//! This module provides the core trait for running diagnostics on Rust projects.

use crate::commands::doctor::reporting::{CheckResult, DiagnosticReport};
use crate::commands::doctor::types::Diagnostic;

/// Weight applied to error diagnostics when calculating health score
#[allow(dead_code)]
pub const ERROR_WEIGHT: u8 = 20;

/// Weight applied to warning diagnostics when calculating health score
#[allow(dead_code)]
pub const WARNING_WEIGHT: u8 = 5;

/// A trait for implementing different types of checks
pub trait Check {
    /// Run the check and return a list of diagnostics
    fn run(&self) -> Vec<Diagnostic>;

    /// Name of the check
    fn name(&self) -> &str;

    /// Description of what this check verifies
    fn description(&self) -> &str;

    /// Category this check belongs to
    fn category(&self) -> &str;
}

/// Run a collection of checks and generate a diagnostic report
pub fn run_checks(checks: Vec<Box<dyn Check>>) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    for check in checks {
        let start = std::time::Instant::now();
        let diagnostics = check.run();
        let duration = start.elapsed();

        let result = CheckResult {
            name: check.name().to_string(),
            description: check.description().to_string(),
            category: check.category().to_string(),
            duration,
            diagnostics,
        };

        report.add_result(result);
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::doctor::types::{Diagnostic, Severity};
    use pretty_assertions::assert_eq;

    struct MockCheck {
        name: String,
        description: String,
        category: String,
        diagnostics: Vec<Diagnostic>,
    }

    impl MockCheck {
        fn new(
            name: impl Into<String>,
            description: impl Into<String>,
            category: impl Into<String>,
            diagnostics: Vec<Diagnostic>,
        ) -> Self {
            Self {
                name: name.into(),
                description: description.into(),
                category: category.into(),
                diagnostics,
            }
        }
    }

    impl Check for MockCheck {
        fn run(&self) -> Vec<Diagnostic> {
            self.diagnostics.clone()
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            &self.description
        }

        fn category(&self) -> &str {
            &self.category
        }
    }

    #[test]
    fn test_diagnostic_creation() {
        let diagnostic = Diagnostic::new(
            "test-check",
            Severity::Warning,
            "A test warning",
            "test-category",
        )
        .with_details("Detailed explanation")
        .with_suggestion("Fix the issue");

        assert_eq!(diagnostic.check_name, "test-check");
        assert_eq!(diagnostic.severity, Severity::Warning);
        assert_eq!(diagnostic.message, "A test warning");
        assert_eq!(diagnostic.details, Some("Detailed explanation".to_string()));
        assert_eq!(diagnostic.suggestion, Some("Fix the issue".to_string()));
        assert_eq!(diagnostic.category, "test-category");
    }

    #[test]
    fn test_report_generation() {
        // Create some test diagnostics
        let error_diag = Diagnostic::new(
            "error-check",
            Severity::Error,
            "An error was found",
            "errors",
        );

        let warning_diag = Diagnostic::new(
            "warning-check",
            Severity::Warning,
            "A warning was found",
            "warnings",
        );

        // Create test checks
        let checks: Vec<Box<dyn Check>> = vec![
            Box::new(MockCheck::new(
                "error-check",
                "Checks for errors",
                "errors",
                vec![error_diag],
            )),
            Box::new(MockCheck::new(
                "warning-check",
                "Checks for warnings",
                "warnings",
                vec![warning_diag],
            )),
        ];

        // Run checks and get report
        let report = run_checks(checks);

        // Verify report
        assert_eq!(report.results.len(), 2);
        assert_eq!(report.diagnostics_by_severity(Severity::Error).len(), 1);
        assert_eq!(report.diagnostics_by_severity(Severity::Warning).len(), 1);

        // Check statistics
        let stats = report.statistics();
        assert_eq!(stats.total_checks, 2);
        assert_eq!(stats.total_diagnostics, 2);
        assert_eq!(stats.errors, 1);
        assert_eq!(stats.warnings, 1);
        assert_eq!(stats.health_score, 100 - (ERROR_WEIGHT + WARNING_WEIGHT));
    }
}
