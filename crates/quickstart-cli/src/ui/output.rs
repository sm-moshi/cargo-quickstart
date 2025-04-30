//! Formatted output utilities for the CLI

use owo_colors::OwoColorize;
use std::io::{self, Write};

/// Print a success message to stdout
#[allow(dead_code)]
pub fn success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

/// Print an info message to stdout
#[allow(dead_code)]
pub fn info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

/// Print a warning message to stderr
#[allow(dead_code)]
pub fn warning(message: &str) {
    eprintln!("{} {}", "⚠".yellow().bold(), message);
}

/// Print an error message to stderr
#[allow(dead_code)]
pub fn error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message);
}

/// Print a header message to stdout
pub fn header(title: &str) {
    println!("\n{}", title.bold().underline());
}

/// Print a section title
pub fn section(title: &str) {
    println!("\n{}", title.bold());
}

/// Print a formatted key-value pair
pub fn key_value(key: &str, value: &str) {
    println!("{}: {}", key.bold(), value);
}

/// Print a list item with a bullet point
#[allow(dead_code)]
pub fn list_item(message: &str) {
    println!("  • {message}");
}

/// Print a confirmation message and get user input
#[allow(dead_code)]
pub fn confirm(message: &str) -> io::Result<bool> {
    print!("{message} [y/N] ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // This is primarily to increase code coverage
    // These functions are simple printing functions that don't return anything
    #[test]
    fn test_output_functions() {
        // Test that these functions don't panic
        header("Test Header");
        section("Test Section");
        key_value("Key", "Value");
        success("Success message");
        info("Info message");
        warning("Warning message");
        error("Error message");
        list_item("List item");

        // We cannot easily test the confirm function in an automated test
        // as it requires stdin interaction. Skip this for coverage purposes.
    }
}
