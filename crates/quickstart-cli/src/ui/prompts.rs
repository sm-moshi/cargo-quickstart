//! Interactive user prompts and input collection

use color_eyre::{eyre::Report, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::fmt::Display;

/// Get the theme for all dialoguer prompts
fn theme() -> ColorfulTheme {
    ColorfulTheme::default()
}

/// Prompt the user for a text input with a default value
pub fn input_with_default(prompt: &str, default: &str) -> Result<String> {
    Ok(Input::with_theme(&theme())
        .with_prompt(prompt)
        .default(default.to_string())
        .interact()?)
}

/// Prompt the user for a text input that must not be empty
#[allow(dead_code)]
pub fn input_required(prompt: &str) -> Result<String> {
    Ok(Input::with_theme(&theme())
        .with_prompt(prompt)
        .allow_empty(false)
        .interact()?)
}

/// Ask the user for confirmation with a default value
pub fn confirm(prompt: &str, default: bool) -> Result<bool> {
    Ok(Confirm::with_theme(&theme())
        .with_prompt(prompt)
        .default(default)
        .interact()?)
}

/// Prompt the user to select from a list of options
pub fn select<T: AsRef<str> + Display>(prompt: &str, options: &[T]) -> Result<usize> {
    Ok(Select::with_theme(&theme())
        .with_prompt(prompt)
        .items(options)
        .default(0)
        .interact()?)
}

/// Prompt the user for a project name, validating it's a valid crate name
#[allow(dead_code)]
pub fn project_name(prompt: &str) -> Result<String> {
    let name = Input::with_theme(&theme())
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), Report> {
            // Validate according to Cargo's rules
            if input.is_empty() {
                return Err(Report::msg("Project name cannot be empty"));
            }

            if input.contains(|c: char| !c.is_alphanumeric() && c != '_' && c != '-') {
                return Err(Report::msg(
                    "Project name must contain only alphanumeric characters, '-', or '_'",
                ));
            }

            if input.chars().next().is_none_or(|c| !c.is_alphabetic()) {
                return Err(Report::msg("Project name must start with a letter"));
            }

            Ok(())
        })
        .interact()?;

    Ok(name)
}
