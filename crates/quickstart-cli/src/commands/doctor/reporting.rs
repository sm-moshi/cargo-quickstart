//! Diagnostic report generation and statistics

use crate::commands::doctor::types::{Diagnostic, Severity};
use std::collections::HashMap;
use std::time::Duration;

/// Constants for health score calculation
pub const ERROR_PENALTY: usize = 20;
pub const WARNING_PENALTY: usize = 5;
pub const SUGGESTION_PENALTY: usize = 1;
pub const MAX_PENALTY: usize = 100;

/// The result of running a diagnostic check
#[derive(Debug)]
pub struct CheckResult {
    /// Name of the check
    pub name: String,
    /// Description of the check
    pub description: String,
    /// Category of the check
    #[allow(dead_code)]
    pub category: String,
    /// Duration of the check execution
    pub duration: Duration,
    /// Diagnostics produced by the check
    pub diagnostics: Vec<Diagnostic>,
}

/// A complete diagnostic report with all check results
#[derive(Debug)]
pub struct DiagnosticReport {
    /// Results of all checks that were run
    pub results: Vec<CheckResult>,
    /// Total duration of all checks
    pub total_duration: Duration,
}

impl DiagnosticReport {
    /// Create a new empty diagnostic report
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            total_duration: Duration::from_secs(0),
        }
    }

    /// Add a check result to the report
    pub fn add_result(&mut self, result: CheckResult) {
        self.total_duration += result.duration;
        self.results.push(result);
    }

    /// Get diagnostics by severity
    pub fn diagnostics_by_severity(&self, severity: Severity) -> Vec<&Diagnostic> {
        self.results
            .iter()
            .flat_map(|r| &r.diagnostics)
            .filter(|d| d.severity == severity)
            .collect()
    }

    /// Get all diagnostics grouped by category
    pub fn diagnostics_by_category(&self) -> HashMap<&str, Vec<&Diagnostic>> {
        let mut map = HashMap::new();

        for result in &self.results {
            for diagnostic in &result.diagnostics {
                map.entry(diagnostic.category.as_str())
                    .or_insert_with(Vec::new)
                    .push(diagnostic);
            }
        }

        map
    }

    /// Calculate health score (0-100) based on diagnostics
    pub fn health_score(&self) -> u8 {
        let error_count = self.diagnostics_by_severity(Severity::Error).len();
        let warning_count = self.diagnostics_by_severity(Severity::Warning).len();
        let suggestion_count = self.diagnostics_by_severity(Severity::Suggestion).len();

        let total_checks = self.results.len();
        if total_checks == 0 {
            return 100;
        }

        // Calculate score using named constants
        let penalty = (error_count * ERROR_PENALTY
            + warning_count * WARNING_PENALTY
            + suggestion_count * SUGGESTION_PENALTY)
            .min(MAX_PENALTY);
        100 - penalty as u8
    }

    /// Get statistics about the diagnostic results
    pub fn statistics(&self) -> DiagnosticStatistics {
        DiagnosticStatistics {
            total_checks: self.results.len(),
            total_diagnostics: self.results.iter().map(|r| r.diagnostics.len()).sum(),
            errors: self.diagnostics_by_severity(Severity::Error).len(),
            warnings: self.diagnostics_by_severity(Severity::Warning).len(),
            suggestions: self.diagnostics_by_severity(Severity::Suggestion).len(),
            info: self.diagnostics_by_severity(Severity::Info).len(),
            health_score: self.health_score(),
        }
    }
}

/// Statistics about a diagnostic report
#[derive(Debug, Clone, Copy)]
pub struct DiagnosticStatistics {
    /// Total number of checks run
    pub total_checks: usize,
    /// Total number of diagnostic findings
    #[allow(dead_code)]
    pub total_diagnostics: usize,
    /// Number of errors found
    pub errors: usize,
    /// Number of warnings found
    pub warnings: usize,
    /// Number of suggestions found
    pub suggestions: usize,
    /// Number of informational messages
    pub info: usize,
    /// Overall health score (0-100)
    pub health_score: u8,
}
