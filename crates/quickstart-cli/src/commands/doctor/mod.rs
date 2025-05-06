//! Doctor command: diagnose project issues and misconfigurations

use crate::ui::output;
use color_eyre::Result;
use std::path::Path;

// Define the module structure
pub mod checks;
pub mod diagnosis;
pub mod reporting;
pub mod types;

// Re-export diagnostic types for use within individual check modules
pub use diagnosis::Check;
pub use types::{Severity, HEALTH_THRESHOLDS};

/// Execute the doctor command
pub fn execute() -> Result<()> {
    output::header("cargo-quickstart Doctor");

    // Create and run checks
    let checks = collect_checks();
    let report = diagnosis::run_checks(checks);

    // Display diagnostics
    for result in &report.results {
        output::section(&format!("Check: {}", result.name));
        output::info(&format!("Description: {}", result.description));
        output::info(&format!("Duration: {:?}", result.duration));

        if result.diagnostics.is_empty() {
            output::success("✅ No issues found");
            continue;
        }

        // Display the diagnostics for this check
        for diagnostic in &result.diagnostics {
            match diagnostic.severity {
                Severity::Info => output::info(&format!("ℹ️ {}", diagnostic.message)),
                Severity::Suggestion => output::info(&format!("💡 {}", diagnostic.message)),
                Severity::Warning => output::warning(&format!("⚠️ {}", diagnostic.message)),
                Severity::Error => output::error(&format!("❌ {}", diagnostic.message)),
            }

            if let Some(details) = &diagnostic.details {
                output::info(&format!("  Details: {details}"));
            }

            if let Some(suggestion) = &diagnostic.suggestion {
                output::info(&format!("  Suggestion: {suggestion}"));
            }
        }
    }

    // Print project health statistics
    let stats = report.statistics();

    output::section("Project Health Summary");
    output::info(&format!("Total checks: {}", stats.total_checks));
    output::info(&format!("Health score: {}%", stats.health_score));
    output::success(&format!("Info: {}", stats.info));
    output::success(&format!("Suggestions: {}", stats.suggestions));
    output::warning(&format!("Warnings: {}", stats.warnings));
    output::error(&format!("Errors: {}", stats.errors));

    // Health assessment
    output::section("Project Health Assessment");
    match stats.health_score {
        score if HEALTH_THRESHOLDS.excellent.contains(&score) => {
            output::success(&format!("Excellent ({}%)", stats.health_score))
        }
        score if HEALTH_THRESHOLDS.good.contains(&score) => {
            output::success(&format!("Good ({}%)", stats.health_score))
        }
        score if HEALTH_THRESHOLDS.moderate.contains(&score) => {
            output::warning(&format!("Moderate ({}%)", stats.health_score))
        }
        score if HEALTH_THRESHOLDS.needs_attention.contains(&score) => {
            output::warning(&format!("Needs attention ({}%)", stats.health_score))
        }
        _ => output::error(&format!("Critical ({}%)", stats.health_score)),
    }

    // Provide recommendations if there are issues
    if stats.warnings > 0 || stats.errors > 0 {
        output::section("Recommendations");

        if stats.errors > 0 {
            output::error("Fix all errors to ensure project functionality.");
        }

        if stats.warnings > 0 {
            output::warning("Address warnings to improve project quality.");
        }

        // Group diagnostics by category
        let categories = report.diagnostics_by_category();
        for (category, diagnostics) in categories {
            let issues_count = diagnostics
                .iter()
                .filter(|d| d.severity == Severity::Warning || d.severity == Severity::Error)
                .count();

            if issues_count > 0 {
                output::info(&format!(
                    "• Check {category} category ({issues_count} issues)"
                ));
            }
        }
    }

    Ok(())
}

/// Collect all diagnostic checks to run
fn collect_checks() -> Vec<Box<dyn Check>> {
    use checks::*;

    let checks: Vec<Box<dyn Check>> = vec![
        Box::new(FilesCheck::new(Path::new("."))),
        Box::new(RustToolchainCheck::new()),
        Box::new(LintsCheck::new()),
        Box::new(DependenciesCheck::new()),
        Box::new(TemplatesCheck::new()),
    ];

    checks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_checks_returns_checks() {
        let checks = collect_checks();
        assert!(
            !checks.is_empty(),
            "collect_checks should return at least one check"
        );
    }

    #[test]
    fn test_execute_runs_without_error() {
        // Skip this test under Miri since it makes file system calls
        if cfg!(miri) {
            eprintln!("Skipping doctor execute test under Miri");
            return;
        }

        let result = execute();
        assert!(result.is_ok(), "execute() should complete without error");
    }
}
