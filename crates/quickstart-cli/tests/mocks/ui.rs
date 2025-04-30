//! Mocks for the UI module

use color_eyre::Result;
use mockall::mock;

// Create a mock for the prompts module
mock! {
    pub Prompts {
        pub fn input_with_default(&self, prompt: &str, default: &str) -> Result<String>;
        pub fn input_required(&self, prompt: &str) -> Result<String>;
        pub fn confirm(&self, prompt: &str, default: bool) -> Result<bool>;
        pub fn select<T: AsRef<str> + std::fmt::Display + 'static>(&self, prompt: &str, options: &[T]) -> Result<usize>;
        pub fn project_name(&self, prompt: &str) -> Result<String>;
    }
}

// Create a mock for the progress module
mock! {
    pub Progress {
        pub fn with_spinner<F: FnOnce() -> Result<T> + 'static, T: 'static>(&self, start_message: &str, success_message: &str, operation: F) -> Result<T>;
    }
}

// Create a mock for the output module
mock! {
    pub Output {
        pub fn header(&self, text: &str);
        pub fn section(&self, text: &str);
        pub fn key_value<T: std::fmt::Display + 'static>(&self, key: &str, value: &T);
        pub fn info(&self, text: &str);
        pub fn success(&self, text: &str);
        pub fn warning(&self, text: &str);
        pub fn error(&self, text: &str);
    }
}
