//! Interactive user prompts and input collection

use color_eyre::{eyre::Report, Result};
use inquire::{validator::Validation, Confirm, CustomUserError, InquireError, Select, Text};

#[cfg(test)]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(test)]
use std::sync::{LazyLock, Mutex};

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

    let result = Text::new(prompt)
        .with_default(default)
        .with_help_message("Enter a value or press Enter to use the default")
        .prompt();

    match result {
        Ok(value) => Ok(value),
        Err(InquireError::OperationCanceled) => Err(Report::msg("Operation canceled by user")),
        Err(err) => Err(Report::msg(format!("Input error: {err}"))),
    }
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

    let result = Text::new(prompt)
        .with_help_message("This field is required")
        .prompt();

    match result {
        Ok(value) => Ok(value),
        Err(InquireError::OperationCanceled) => Err(Report::msg("Operation canceled by user")),
        Err(err) => Err(Report::msg(format!("Input error: {err}"))),
    }
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

    let result = Confirm::new(prompt)
        .with_default(default)
        .with_help_message("Press y for yes, n for no")
        .prompt();

    match result {
        Ok(value) => Ok(value),
        Err(InquireError::OperationCanceled) => Err(Report::msg("Operation canceled by user")),
        Err(err) => Err(Report::msg(format!("Confirmation error: {err}"))),
    }
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

    let result = Select::new(prompt, options.to_vec())
        .with_help_message("Use arrow keys to navigate, Enter to select")
        .prompt();

    match result {
        Ok(value) => {
            // Find the index of the selected option
            match options.iter().position(|&item| item == value) {
                Some(index) => Ok(index),
                None => Err(Report::msg("Selected value not found in options list")),
            }
        }
        Err(InquireError::OperationCanceled) => Err(Report::msg("Operation canceled by user")),
        Err(err) => Err(Report::msg(format!("Selection error: {err}"))),
    }
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

    let validator = |input: &str| -> Result<Validation, CustomUserError> {
        // Validate according to Cargo's rules
        if input.is_empty() {
            return Ok(Validation::Invalid("Project name cannot be empty".into()));
        }

        if input.contains(|c: char| !c.is_alphanumeric() && c != '_' && c != '-') {
            return Ok(Validation::Invalid(
                "Project name must contain only alphanumeric characters, '-', or '_'".into(),
            ));
        }

        if input.chars().next().is_none_or(|c| !c.is_alphabetic()) {
            return Ok(Validation::Invalid(
                "Project name must start with a letter".into(),
            ));
        }

        Ok(Validation::Valid)
    };

    let result = Text::new(prompt)
        .with_validator(validator)
        .with_help_message(
            "Must start with a letter and contain only alphanumeric characters, '-', or '_'",
        )
        .prompt();

    match result {
        Ok(value) => Ok(value),
        Err(InquireError::OperationCanceled) => Err(Report::msg("Operation canceled by user")),
        Err(err) => Err(Report::msg(format!("Input error: {err}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::Result;
    use pretty_assertions::assert_eq;

    // Helper struct for RAII-style mocking management
    struct MockGuard;

    impl MockGuard {
        fn new() -> Self {
            enable_mocking();
            Self
        }
    }

    impl Drop for MockGuard {
        fn drop(&mut self) {
            disable_mocking();
        }
    }

    #[test]
    fn test_input_with_default_mocked() -> Result<()> {
        let _guard = MockGuard::new();
        set_mock_input(Some("test input".to_string()));

        let result = input_with_default("Test prompt", "default")?;
        assert_eq!(result, "test input");

        Ok(())
    }

    #[test]
    fn test_input_required_mocked() -> Result<()> {
        let _guard = MockGuard::new();
        set_mock_input(Some("required input".to_string()));

        let result = input_required("Test prompt")?;
        assert_eq!(result, "required input");

        Ok(())
    }

    #[test]
    fn test_confirm_mocked() -> Result<()> {
        let _guard = MockGuard::new();
        set_mock_confirm(Some(true));

        let result = confirm("Test prompt", false)?;
        assert!(result);

        Ok(())
    }

    #[test]
    fn test_select_mocked() -> Result<()> {
        let _guard = MockGuard::new();
        set_mock_select(Some(1));

        let result = select("Test prompt", &["Option 1", "Option 2"])?;
        assert_eq!(result, 1);

        Ok(())
    }

    #[test]
    fn test_project_name_validation() -> Result<()> {
        let _guard = MockGuard::new();

        // Test valid project name
        set_mock_input(Some("valid-project".to_string()));
        let result = project_name("Project name")?;
        assert_eq!(result, "valid-project");

        // Test empty project name
        set_mock_input(Some("".to_string()));
        let empty_result = project_name("Project name");
        assert!(empty_result.is_err());

        // Test invalid characters
        set_mock_input(Some("invalid@project".to_string()));
        let invalid_chars_result = project_name("Project name");
        assert!(invalid_chars_result.is_err());

        // Test non-alphabetic first character
        set_mock_input(Some("1invalid".to_string()));
        let invalid_start_result = project_name("Project name");
        assert!(invalid_start_result.is_err());

        Ok(())
    }
}
