//! Trait definition and implementations for inquire API abstraction
//! This allows for dependency injection and better testing capabilities

use color_eyre::{eyre::Report, Result};
use inquire::validator::Validation;
use inquire::{Confirm, MultiSelect, Select, Text};
#[cfg(test)]
use std::cell::RefCell;
use std::fmt::Display;

/// Trait for abstracting inquire functionality
/// This enables dependency injection for testing
pub trait InquireApi {
    /// Prompts the user for text input
    fn text(&self, message: &str, help: Option<&str>) -> Result<String>;

    /// Prompts the user for text input with validation
    fn text_with_validation(
        &self,
        message: &str,
        help: Option<&str>,
        validator: impl Fn(&str) -> Result<bool, String> + Clone + 'static,
        error_message: &str,
    ) -> Result<String>;

    /// Prompts the user to select one option from a list
    fn select<T: ToString + Display + Clone>(
        &self,
        message: &str,
        options: &[T],
        help: Option<&str>,
    ) -> Result<usize>;

    /// Prompts the user for a yes/no confirmation
    fn confirm(&self, message: &str, default: bool) -> Result<bool>;

    /// Prompts the user to select multiple options from a list
    fn multiselect<T: ToString + Display + Clone>(
        &self,
        message: &str,
        options: &[T],
        defaults: &[usize],
        help: Option<&str>,
    ) -> Result<Vec<usize>>;
}

/// Real implementation that uses the inquire crate directly
pub struct RealInquire;

impl InquireApi for RealInquire {
    fn text(&self, message: &str, help: Option<&str>) -> Result<String> {
        let mut prompt = Text::new(message);

        if let Some(h) = help {
            prompt = prompt.with_help_message(h);
        }

        prompt
            .prompt()
            .map_err(|e| Report::msg(format!("Text input error: {e}")))
    }

    fn text_with_validation(
        &self,
        message: &str,
        help: Option<&str>,
        validator: impl Fn(&str) -> Result<bool, String> + Clone + 'static,
        error_message: &str,
    ) -> Result<String> {
        let error_msg = error_message.to_string();
        let validation_fn = move |input: &str| match validator(input) {
            Ok(true) => Ok(Validation::Valid),
            Ok(false) | Err(_) => Ok(Validation::Invalid(error_msg.clone().into())),
        };

        let mut prompt = Text::new(message);
        prompt = prompt.with_validator(validation_fn);

        if let Some(h) = help {
            prompt = prompt.with_help_message(h);
        }

        prompt
            .prompt()
            .map_err(|e| Report::msg(format!("Text input error: {e}")))
    }

    fn select<T: ToString + Display + Clone>(
        &self,
        message: &str,
        options: &[T],
        help: Option<&str>,
    ) -> Result<usize> {
        let mut prompt = Select::new(message, options.to_vec());

        if let Some(h) = help {
            prompt = prompt.with_help_message(h);
        }

        prompt
            .prompt()
            .map_err(|e| Report::msg(format!("Selection error: {e}")))
            .map(|selected| {
                // Find the index of the selected option
                options
                    .iter()
                    .position(|item| item.to_string() == selected.to_string())
                    .unwrap_or(0) // Default to first option if somehow not found
            })
    }

    fn confirm(&self, message: &str, default: bool) -> Result<bool> {
        Confirm::new(message)
            .with_default(default)
            .prompt()
            .map_err(|e| Report::msg(format!("Confirmation error: {e}")))
    }

    fn multiselect<T: ToString + Display + Clone>(
        &self,
        message: &str,
        options: &[T],
        defaults: &[usize],
        help: Option<&str>,
    ) -> Result<Vec<usize>> {
        let mut prompt = MultiSelect::new(message, options.to_vec());

        if !defaults.is_empty() {
            prompt = prompt.with_default(defaults);
        }

        if let Some(h) = help {
            prompt = prompt.with_help_message(h);
        }

        prompt
            .prompt()
            .map_err(|e| Report::msg(format!("MultiSelect error: {e}")))
            .map(|selected| {
                // Map selected items back to their indices
                selected
                    .iter()
                    .filter_map(|item| {
                        options
                            .iter()
                            .position(|opt| opt.to_string() == item.to_string())
                    })
                    .collect()
            })
    }
}

/// Test implementation for use in unit tests
#[cfg(test)]
pub struct TestInquire {
    text_responses: RefCell<std::collections::VecDeque<Result<String>>>,
    select_responses: RefCell<std::collections::VecDeque<Result<usize>>>,
    confirm_responses: RefCell<std::collections::VecDeque<Result<bool>>>,
    multiselect_responses: RefCell<std::collections::VecDeque<Result<Vec<usize>>>>,
}

#[cfg(test)]
impl TestInquire {
    /// Create a new instance with empty response queues
    pub fn new() -> Self {
        Self {
            text_responses: RefCell::new(std::collections::VecDeque::new()),
            select_responses: RefCell::new(std::collections::VecDeque::new()),
            confirm_responses: RefCell::new(std::collections::VecDeque::new()),
            multiselect_responses: RefCell::new(std::collections::VecDeque::new()),
        }
    }

    /// Queue a text response to be returned on next text call
    pub fn queue_text_response(&self, response: Result<String>) {
        self.text_responses.borrow_mut().push_back(response);
    }

    /// Queue a select response to be returned on next select call
    pub fn queue_select_response(&self, response: Result<usize>) {
        self.select_responses.borrow_mut().push_back(response);
    }

    /// Queue a confirm response to be returned on next confirm call
    pub fn queue_confirm_response(&self, response: Result<bool>) {
        self.confirm_responses.borrow_mut().push_back(response);
    }

    /// Queue a multiselect response to be returned on next multiselect call
    pub fn queue_multiselect_response(&self, response: Result<Vec<usize>>) {
        self.multiselect_responses.borrow_mut().push_back(response);
    }

    /// Helper methods to easily add successful responses
    pub fn add_text(&self, text: &str) {
        self.queue_text_response(Ok(text.to_string()));
    }

    pub fn add_select(&self, idx: usize) {
        self.queue_select_response(Ok(idx));
    }

    pub fn add_confirm(&self, value: bool) {
        self.queue_confirm_response(Ok(value));
    }

    pub fn add_multiselect(&self, indices: Vec<usize>) {
        self.queue_multiselect_response(Ok(indices));
    }

    /// Helper method to consume and return the next text response
    fn next_text_response(&self) -> Result<String> {
        self.text_responses
            .borrow_mut()
            .pop_front()
            .unwrap_or_else(|| Err(Report::msg("No more mock text responses")))
    }

    /// Helper method to consume and return the next select response
    fn next_select_response(&self) -> Result<usize> {
        self.select_responses
            .borrow_mut()
            .pop_front()
            .unwrap_or_else(|| Err(Report::msg("No more mock select responses")))
    }

    /// Helper method to consume and return the next confirm response
    fn next_confirm_response(&self) -> Result<bool> {
        self.confirm_responses
            .borrow_mut()
            .pop_front()
            .unwrap_or_else(|| Err(Report::msg("No more mock confirm responses")))
    }

    /// Helper method to consume and return the next multiselect response
    fn next_multiselect_response(&self) -> Result<Vec<usize>> {
        self.multiselect_responses
            .borrow_mut()
            .pop_front()
            .unwrap_or_else(|| Err(Report::msg("No more mock multiselect responses")))
    }
}

#[cfg(test)]
impl InquireApi for TestInquire {
    fn text(&self, _message: &str, _help: Option<&str>) -> Result<String> {
        self.next_text_response()
    }

    fn text_with_validation(
        &self,
        _message: &str,
        _help: Option<&str>,
        validator: impl Fn(&str) -> Result<bool, String> + Clone + 'static,
        error_message: &str,
    ) -> Result<String> {
        let response = self.next_text_response()?;

        // Apply validation
        if let Ok(valid) = validator(&response) {
            if valid {
                Ok(response)
            } else {
                Err(Report::msg(format!("Validation failed: {error_message}")))
            }
        } else {
            Err(Report::msg(format!("Validation failed: {error_message}")))
        }
    }

    fn select<T: ToString + Display + Clone>(
        &self,
        _message: &str,
        _options: &[T],
        _help: Option<&str>,
    ) -> Result<usize> {
        self.next_select_response()
    }

    fn confirm(&self, _message: &str, _default: bool) -> Result<bool> {
        self.next_confirm_response()
    }

    fn multiselect<T: ToString + Display + Clone>(
        &self,
        _message: &str,
        _options: &[T],
        _defaults: &[usize],
        _help: Option<&str>,
    ) -> Result<Vec<usize>> {
        self.next_multiselect_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_inquire_text() {
        let test_inquire = TestInquire::new();

        // Setup test data
        test_inquire.add_text("sample text");

        // Test
        let result = test_inquire.text("Test prompt", None);
        assert!(result.is_ok());
        if let Ok(text) = result {
            assert_eq!(text, "sample text");
        } else {
            panic!("Result should be Ok but was Err");
        }
    }

    #[test]
    fn test_test_inquire_text_error() {
        let test_inquire = TestInquire::new();

        // No responses queued, should return an error
        let result = test_inquire.text("Test prompt", None);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No more mock text responses"));
    }

    #[test]
    fn test_test_inquire_text_with_validation() {
        let test_inquire = TestInquire::new();

        // Setup test data
        test_inquire.add_text("valid-text");

        // Create a validator
        let validator = |input: &str| -> Result<bool, String> {
            if input.is_empty() {
                Ok(false)
            } else {
                Ok(true)
            }
        };

        // Test with validator - should pass
        let result = test_inquire.text_with_validation(
            "Test prompt",
            None,
            validator,
            "Input cannot be empty",
        );
        assert!(result.is_ok());
        if let Ok(text) = result {
            assert_eq!(text, "valid-text");
        } else {
            panic!("Result should be Ok but was Err");
        }

        // Test with empty input - should fail validation
        test_inquire.add_text("");
        let result = test_inquire.text_with_validation(
            "Test prompt",
            None,
            validator,
            "Input cannot be empty",
        );
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("Validation failed"));
        } else {
            panic!("Result should be Err but was Ok");
        }
    }

    #[test]
    fn test_test_inquire_text_with_validation_error_response() {
        let test_inquire = TestInquire::new();

        // Setup test data with an error
        test_inquire.queue_text_response(Err(Report::msg("Mock text error")));

        let validator = |_: &str| -> Result<bool, String> { Ok(true) };

        // Should propagate the error from the response
        let result = test_inquire.text_with_validation(
            "Test prompt",
            None,
            validator,
            "Input cannot be empty",
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Mock text error"));
    }

    #[test]
    fn test_test_inquire_text_with_validation_error_from_validator() {
        let test_inquire = TestInquire::new();

        // Setup test data
        test_inquire.add_text("invalid");

        // Validator that returns an Err result
        let validator = |_: &str| -> Result<bool, String> { Err("Validator error".to_string()) };

        // Should return an error when validator returns Err
        let result = test_inquire.text_with_validation(
            "Test prompt",
            None,
            validator,
            "Custom error message",
        );
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Validation failed"));
    }

    #[test]
    fn test_test_inquire_select() {
        let test_inquire = TestInquire::new();

        // Setup test data
        test_inquire.add_select(1);

        // Test select
        let options = ["Option 1", "Option 2", "Option 3"];
        let result = test_inquire.select("Test prompt", &options, None);
        assert!(result.is_ok());
        if let Ok(selected) = result {
            assert_eq!(selected, 1);
        } else {
            panic!("Result should be Ok but was Err");
        }
    }

    #[test]
    fn test_test_inquire_select_error() {
        let test_inquire = TestInquire::new();

        // No responses queued, should return an error
        let options = ["Option 1", "Option 2"];
        let result = test_inquire.select("Test prompt", &options, None);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No more mock select responses"));
    }

    #[test]
    fn test_test_inquire_select_with_help() {
        let test_inquire = TestInquire::new();

        // Setup test data
        test_inquire.add_select(2);

        // Test select with help text
        let options = ["Option 1", "Option 2", "Option 3"];
        let result = test_inquire.select("Test prompt", &options, Some("Help text"));
        assert!(result.is_ok());
        if let Ok(selected) = result {
            assert_eq!(selected, 2);
        } else {
            panic!("Result should be Ok but was Err");
        }
    }

    #[test]
    fn test_test_inquire_confirm() {
        let test_inquire = TestInquire::new();

        // Setup test data
        test_inquire.add_confirm(true);

        // Test confirm
        let result = test_inquire.confirm("Test prompt", false);
        assert!(result.is_ok());
        if let Ok(confirmed) = result {
            assert!(confirmed);
        } else {
            panic!("Result should be Ok but was Err");
        }
    }

    #[test]
    fn test_test_inquire_confirm_error() {
        let test_inquire = TestInquire::new();

        // No responses queued, should return an error
        let result = test_inquire.confirm("Test prompt", true);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No more mock confirm responses"));
    }

    #[test]
    fn test_test_inquire_confirm_false() {
        let test_inquire = TestInquire::new();

        // Setup test data for false response
        test_inquire.add_confirm(false);

        // Test confirm with default=true but response=false
        let result = test_inquire.confirm("Test prompt", true);
        assert!(result.is_ok());
        if let Ok(confirmed) = result {
            assert!(!confirmed);
        } else {
            panic!("Result should be Ok but was Err");
        }
    }

    #[test]
    fn test_test_inquire_multiselect() {
        let test_inquire = TestInquire::new();

        // Setup test data
        test_inquire.add_multiselect(vec![0, 2]);

        // Test multiselect
        let options = ["Option 1", "Option 2", "Option 3"];
        let result = test_inquire.multiselect("Test prompt", &options, &[], None);
        assert!(result.is_ok());
        if let Ok(selected) = result {
            assert_eq!(selected, vec![0, 2]);
        } else {
            panic!("Result should be Ok but was Err");
        }
    }

    #[test]
    fn test_test_inquire_multiselect_error() {
        let test_inquire = TestInquire::new();

        // No responses queued, should return an error
        let options = ["Option 1", "Option 2"];
        let result = test_inquire.multiselect("Test prompt", &options, &[], None);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No more mock multiselect responses"));
    }

    #[test]
    fn test_test_inquire_multiselect_with_defaults_and_help() {
        let test_inquire = TestInquire::new();

        // Setup test data
        test_inquire.add_multiselect(vec![1, 3]);

        // Test multiselect with defaults and help
        let options = ["Option 1", "Option 2", "Option 3", "Option 4"];
        let result = test_inquire.multiselect(
            "Test prompt",
            &options,
            &[0, 2], // Default selections
            Some("Help text"),
        );
        assert!(result.is_ok());
        if let Ok(selected) = result {
            assert_eq!(selected, vec![1, 3]);
        } else {
            panic!("Result should be Ok but was Err");
        }
    }

    #[test]
    fn test_test_inquire_queue_error_responses() {
        let test_inquire = TestInquire::new();

        // Queue error responses for each type
        test_inquire.queue_text_response(Err(Report::msg("Text error")));
        test_inquire.queue_select_response(Err(Report::msg("Select error")));
        test_inquire.queue_confirm_response(Err(Report::msg("Confirm error")));
        test_inquire.queue_multiselect_response(Err(Report::msg("Multiselect error")));

        // Test each error response
        assert!(test_inquire
            .text("", None)
            .unwrap_err()
            .to_string()
            .contains("Text error"));

        let options = ["A", "B"];
        assert!(test_inquire
            .select("", &options, None)
            .unwrap_err()
            .to_string()
            .contains("Select error"));

        assert!(test_inquire
            .confirm("", false)
            .unwrap_err()
            .to_string()
            .contains("Confirm error"));

        assert!(test_inquire
            .multiselect("", &options, &[], None)
            .unwrap_err()
            .to_string()
            .contains("Multiselect error"));
    }

    #[test]
    fn test_test_inquire_empty_queue_handling() {
        let test_inquire = TestInquire::new();

        // Add a single response of each type
        test_inquire.add_text("one text");
        test_inquire.add_select(0);
        test_inquire.add_confirm(true);
        test_inquire.add_multiselect(vec![1]);

        // Consume all responses
        let _text = test_inquire.text("", None);
        let options = ["A", "B"];
        let _select = test_inquire.select("", &options, None);
        let _confirm = test_inquire.confirm("", false);
        let _multi = test_inquire.multiselect("", &options, &[], None);

        // Verify we get errors for empty queues
        assert!(test_inquire.text("", None).is_err());
        assert!(test_inquire.select("", &options, None).is_err());
        assert!(test_inquire.confirm("", false).is_err());
        assert!(test_inquire.multiselect("", &options, &[], None).is_err());
    }

    #[test]
    fn test_test_inquire_multiple_text_responses() {
        let test_inquire = TestInquire::new();

        // Queue multiple responses
        test_inquire.add_text("first response");
        test_inquire.add_text("second response");
        test_inquire.add_text("third response");

        // Test them in order
        let result1 = test_inquire.text("Prompt 1", None);
        assert!(result1.is_ok());
        if let Ok(text) = result1 {
            assert_eq!(text, "first response");
        }

        let result2 = test_inquire.text("Prompt 2", Some("Help"));
        assert!(result2.is_ok());
        if let Ok(text) = result2 {
            assert_eq!(text, "second response");
        }

        let result3 = test_inquire.text("Prompt 3", None);
        assert!(result3.is_ok());
        if let Ok(text) = result3 {
            assert_eq!(text, "third response");
        }
    }

    #[test]
    fn test_test_inquire_with_help_text() {
        let test_inquire = TestInquire::new();

        // Add a response
        test_inquire.add_text("response with help");

        // Test that help parameter is passed correctly (although ignored in the mock)
        let result = test_inquire.text("Test prompt", Some("This is helpful text"));
        assert!(result.is_ok());
        if let Ok(text) = result {
            assert_eq!(text, "response with help");
        }
    }

    #[test]
    fn test_test_inquire_validation_with_help() {
        let test_inquire = TestInquire::new();

        // Add a valid response
        test_inquire.add_text("valid input");

        // Create a validator that always passes
        let validator = |_: &str| -> Result<bool, String> { Ok(true) };

        // Test with help message
        let result = test_inquire.text_with_validation(
            "Test prompt",
            Some("Helper text"),
            validator,
            "Not applicable",
        );

        assert!(result.is_ok());
        if let Ok(text) = result {
            assert_eq!(text, "valid input");
        }
    }

    #[test]
    fn test_test_inquire_text_explicit_ok() {
        let test_inquire = TestInquire::new();

        // Queue an explicit Ok response
        test_inquire.queue_text_response(Ok("explicit ok".to_string()));

        // Test that it works the same as add_text
        let result = test_inquire.text("Test", None);
        assert!(result.is_ok());
        if let Ok(text) = result {
            assert_eq!(text, "explicit ok");
        }
    }

    #[test]
    fn test_test_inquire_mixed_errors_and_successes() {
        let test_inquire = TestInquire::new();

        // Queue mixed responses
        test_inquire.add_text("success");
        test_inquire.queue_text_response(Err(Report::msg("First error")));
        test_inquire.add_text("another success");

        // Test the sequence
        let result1 = test_inquire.text("", None);
        assert!(result1.is_ok());
        if let Ok(text) = result1 {
            assert_eq!(text, "success");
        }

        let err_result = test_inquire.text("", None);
        assert!(err_result.is_err());
        if let Err(err) = err_result {
            assert!(err.to_string().contains("First error"));
        }

        let result3 = test_inquire.text("", None);
        assert!(result3.is_ok());
        if let Ok(text) = result3 {
            assert_eq!(text, "another success");
        }
    }
}
