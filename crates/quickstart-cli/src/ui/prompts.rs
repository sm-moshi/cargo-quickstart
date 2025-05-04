//! Interactive user prompts and input collection

use color_eyre::{eyre::Report, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

#[cfg(test)]
use std::sync::{LazyLock, Mutex};
#[cfg(test)]
use std::sync::atomic::{AtomicBool, Ordering};

// Global mock state
#[cfg(test)]
static MOCK_ENABLED: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
#[cfg(test)]
static MOCK_INPUT: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
#[cfg(test)]
static MOCK_CONFIRM: LazyLock<Mutex<Option<bool>>> = LazyLock::new(|| Mutex::new(None));
#[cfg(test)]
static MOCK_SELECT: LazyLock<Mutex<Option<usize>>> = LazyLock::new(|| Mutex::new(None));

// Mock control functions
#[cfg(test)]
pub fn enable_mocking() {
    MOCK_ENABLED.store(true, Ordering::SeqCst);
}

#[cfg(test)]
pub fn disable_mocking() {
    MOCK_ENABLED.store(false, Ordering::SeqCst);
}

#[cfg(test)]
pub fn set_mock_input(value: Option<String>) {
    if let Ok(mut lock) = MOCK_INPUT.lock() {
        *lock = value;
    }
}

#[cfg(test)]
pub fn set_mock_confirm(value: Option<bool>) {
    if let Ok(mut lock) = MOCK_CONFIRM.lock() {
        *lock = value;
    }
}

#[cfg(test)]
pub fn set_mock_select(value: Option<usize>) {
    if let Ok(mut lock) = MOCK_SELECT.lock() {
        *lock = value;
    }
}

/// Get the theme for all dialoguer prompts
fn theme() -> ColorfulTheme {
    ColorfulTheme::default()
}

/// Get user input with a default value
pub fn input_with_default(prompt: &str, default: &str) -> color_eyre::Result<String> {
    #[cfg(test)]
    {
        if MOCK_ENABLED.load(Ordering::SeqCst) {
            if let Ok(mut lock) = MOCK_INPUT.lock() {
                if let Some(mock_value) = lock.take() {
                    return Ok(mock_value);
                }
            }
        }
    }

    Ok(Input::with_theme(&theme())
        .with_prompt(prompt)
        .default(default.into())
        .interact()?)
}

/// Get required user input (no default)
#[allow(dead_code)]
pub fn input_required(prompt: &str) -> color_eyre::Result<String> {
    #[cfg(test)]
    {
        if MOCK_ENABLED.load(Ordering::SeqCst) {
            if let Ok(mut lock) = MOCK_INPUT.lock() {
                if let Some(mock_value) = lock.take() {
                    return Ok(mock_value);
                }
            }
        }
    }

    Ok(Input::with_theme(&theme()).with_prompt(prompt).interact()?)
}

/// Get user confirmation
pub fn confirm(prompt: &str, default: bool) -> color_eyre::Result<bool> {
    #[cfg(test)]
    {
        if MOCK_ENABLED.load(Ordering::SeqCst) {
            if let Ok(mut lock) = MOCK_CONFIRM.lock() {
                if let Some(mock_value) = lock.take() {
                    return Ok(mock_value);
                }
            }
        }
    }

    Ok(Confirm::with_theme(&theme())
        .with_prompt(prompt)
        .default(default)
        .interact()?)
}

/// Get user selection from a list of options
pub fn select(prompt: &str, options: &[&str]) -> color_eyre::Result<usize> {
    #[cfg(test)]
    {
        if MOCK_ENABLED.load(Ordering::SeqCst) {
            if let Ok(mut lock) = MOCK_SELECT.lock() {
                if let Some(mock_value) = lock.take() {
                    return Ok(mock_value);
                }
            }
        }
    }

    Ok(Select::with_theme(&theme())
        .with_prompt(prompt)
        .items(options)
        .interact()?)
}

/// Prompt the user for a project name, validating it's a valid crate name
#[allow(dead_code)]
pub fn project_name(prompt: &str) -> Result<String> {
    #[cfg(test)]
    {
        if MOCK_ENABLED.load(Ordering::SeqCst) {
            if let Ok(mut lock) = MOCK_INPUT.lock() {
                if let Some(mock_value) = lock.take() {
                    // Validate the mock input
                    if mock_value.is_empty() {
                        return Err(Report::msg("Project name cannot be empty"));
                    }

                    if mock_value.contains(|c: char| !c.is_alphanumeric() && c != '_' && c != '-') {
                        return Err(Report::msg(
                            "Project name must contain only alphanumeric characters, '-', or '_'",
                        ));
                    }

                    if mock_value.chars().next().is_none_or(|c| !c.is_alphabetic()) {
                        return Err(Report::msg("Project name must start with a letter"));
                    }

                    return Ok(mock_value);
                }
            }
        }
    }

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

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_input_with_default_mocked() {
        enable_mocking();
        set_mock_input(Some("test input".to_string()));

        let result = input_with_default("Test prompt", "default").unwrap();
        assert_eq!(result, "test input");

        disable_mocking();
    }

    #[test]
    fn test_input_required_mocked() {
        enable_mocking();
        set_mock_input(Some("required input".to_string()));

        let result = input_required("Test prompt").unwrap();
        assert_eq!(result, "required input");

        disable_mocking();
    }

    #[test]
    fn test_confirm_mocked() {
        enable_mocking();
        set_mock_confirm(Some(true));

        let result = confirm("Test prompt", false).unwrap();
        assert!(result);

        disable_mocking();
    }

    #[test]
    fn test_select_mocked() {
        enable_mocking();
        set_mock_select(Some(1));

        let result = select("Test prompt", &["Option 1", "Option 2"]).unwrap();
        assert_eq!(result, 1);

        disable_mocking();
    }

    #[test]
    fn test_project_name_validation() {
        enable_mocking();

        // Test valid project name
        set_mock_input(Some("valid-project".to_string()));
        let result = project_name("Project name");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "valid-project");

        // Test empty project name
        set_mock_input(Some("".to_string()));
        let result = project_name("Project name");
        assert!(result.is_err());

        // Test invalid characters
        set_mock_input(Some("invalid!project".to_string()));
        let result = project_name("Project name");
        assert!(result.is_err());

        // Test invalid start character
        set_mock_input(Some("1invalid".to_string()));
        let result = project_name("Project name");
        assert!(result.is_err());

        disable_mocking();
    }
}
