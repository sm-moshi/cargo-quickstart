//! Core diagnostic types for the doctor command

use std::fmt;
use std::ops::RangeInclusive;

/// Health score thresholds for project assessment
#[allow(dead_code)]
pub struct HealthScoreThresholds {
    pub excellent: RangeInclusive<u8>,
    pub good: RangeInclusive<u8>,
    pub moderate: RangeInclusive<u8>,
    pub needs_attention: RangeInclusive<u8>,
}

/// Default health score thresholds
#[allow(dead_code)]
pub const HEALTH_THRESHOLDS: HealthScoreThresholds = HealthScoreThresholds {
    excellent: 90..=100,
    good: 70..=89,
    moderate: 50..=69,
    needs_attention: 30..=49,
};

/// Represents the severity level of a diagnostic
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Informational message, no action needed
    Info,
    /// Suggestion that may improve the project
    Suggestion,
    /// Warning that should be addressed
    Warning,
    /// Error that needs to be fixed
    Error,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "Info"),
            Severity::Suggestion => write!(f, "Suggestion"),
            Severity::Warning => write!(f, "Warning"),
            Severity::Error => write!(f, "Error"),
        }
    }
}

/// A diagnostic finding from a check
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Name of the check that produced this diagnostic
    #[allow(dead_code)]
    pub check_name: String,
    /// Severity of the diagnostic
    pub severity: Severity,
    /// Short message describing the issue
    pub message: String,
    /// Optional detailed description
    pub details: Option<String>,
    /// Suggested fix or mitigation
    pub suggestion: Option<String>,
    /// Category this diagnostic belongs to
    pub category: String,
}

impl Diagnostic {
    /// Create a new diagnostic
    pub fn new(
        check_name: impl Into<String>,
        severity: Severity,
        message: impl Into<String>,
        category: impl Into<String>,
    ) -> Self {
        Self {
            check_name: check_name.into(),
            severity,
            message: message.into(),
            details: None,
            suggestion: None,
            category: category.into(),
        }
    }

    /// Add detailed description to the diagnostic
    #[allow(dead_code)]
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// Add a suggestion for fixing the issue
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_health_thresholds_constants() {
        // Test that the threshold ranges are correct and non-overlapping
        assert_eq!(*HEALTH_THRESHOLDS.excellent.start(), 90);
        assert_eq!(*HEALTH_THRESHOLDS.excellent.end(), 100);

        assert_eq!(*HEALTH_THRESHOLDS.good.start(), 70);
        assert_eq!(*HEALTH_THRESHOLDS.good.end(), 89);

        assert_eq!(*HEALTH_THRESHOLDS.moderate.start(), 50);
        assert_eq!(*HEALTH_THRESHOLDS.moderate.end(), 69);

        assert_eq!(*HEALTH_THRESHOLDS.needs_attention.start(), 30);
        assert_eq!(*HEALTH_THRESHOLDS.needs_attention.end(), 49);
    }

    #[test]
    fn test_severity_display() {
        assert_eq!(Severity::Info.to_string(), "Info");
        assert_eq!(Severity::Suggestion.to_string(), "Suggestion");
        assert_eq!(Severity::Warning.to_string(), "Warning");
        assert_eq!(Severity::Error.to_string(), "Error");
    }

    #[test]
    fn test_severity_ordering() {
        // Test that severities are correctly ordered
        assert!(Severity::Info < Severity::Suggestion);
        assert!(Severity::Suggestion < Severity::Warning);
        assert!(Severity::Warning < Severity::Error);

        // Test equality
        assert_eq!(Severity::Info, Severity::Info);
        assert_eq!(Severity::Suggestion, Severity::Suggestion);
        assert_eq!(Severity::Warning, Severity::Warning);
        assert_eq!(Severity::Error, Severity::Error);
    }

    #[test]
    fn test_diagnostic_creation_basic() {
        let diagnostic = Diagnostic::new(
            "test-check",
            Severity::Warning,
            "A test warning",
            "test-category",
        );

        assert_eq!(diagnostic.check_name, "test-check");
        assert_eq!(diagnostic.severity, Severity::Warning);
        assert_eq!(diagnostic.message, "A test warning");
        assert_eq!(diagnostic.category, "test-category");
        assert_eq!(diagnostic.details, None);
        assert_eq!(diagnostic.suggestion, None);
    }

    #[test]
    fn test_diagnostic_with_details() {
        let diagnostic = Diagnostic::new(
            "test-check",
            Severity::Warning,
            "A test warning",
            "test-category",
        )
        .with_details("Detailed explanation");

        assert_eq!(diagnostic.details, Some("Detailed explanation".to_string()));
    }

    #[test]
    fn test_diagnostic_with_suggestion() {
        let diagnostic = Diagnostic::new(
            "test-check",
            Severity::Warning,
            "A test warning",
            "test-category",
        )
        .with_suggestion("Fix the issue");

        assert_eq!(diagnostic.suggestion, Some("Fix the issue".to_string()));
    }

    #[test]
    fn test_diagnostic_full_chain() {
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
}
