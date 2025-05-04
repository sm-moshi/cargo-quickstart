//! Formatted output utilities for the CLI

use console::Style;
use std::io::{self, Write};

/// Print a success message to stdout
#[allow(dead_code)]
pub fn success(message: &str) {
    let style = Style::new().green().bold();
    println!("{} {}", style.apply_to("✓"), message);
}

/// Print an info message to stdout
#[allow(dead_code)]
pub fn info(message: &str) {
    let style = Style::new().blue().bold();
    println!("{} {}", style.apply_to("ℹ"), message);
}

/// Print a warning message to stderr
#[allow(dead_code)]
pub fn warning(message: &str) {
    let style = Style::new().yellow().bold();
    eprintln!("{} {}", style.apply_to("⚠"), message);
}

/// Print an error message to stderr
#[allow(dead_code)]
pub fn error(message: &str) {
    let style = Style::new().red().bold();
    eprintln!("{} {}", style.apply_to("✗"), message);
}

/// Print a header message to stdout
pub fn header(title: &str) {
    let style = Style::new().bold().underlined();
    println!("\n{}", style.apply_to(title));
}

/// Print a section title
pub fn section(title: &str) {
    let style = Style::new().bold();
    println!("\n{}", style.apply_to(title));
}

/// Print a formatted key-value pair
pub fn key_value(key: &str, value: &str) {
    let style = Style::new().bold();
    println!("{}: {}", style.apply_to(key), value);
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
