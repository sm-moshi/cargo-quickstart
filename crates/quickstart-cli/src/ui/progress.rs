//! Progress indicators for long-running operations

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;

/// Create a spinner for tasks that have an unknown duration
pub fn spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

/// Create a progress bar for tasks with a known number of steps
#[allow(dead_code)]
pub fn progress_bar(total: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("=> "),
    );
    pb.set_message(message.to_string());
    pb
}

/// Create a multi-progress display for showing multiple progress indicators
#[allow(dead_code)]
pub fn multi_progress() -> MultiProgress {
    MultiProgress::new()
}

/// Wrap a function with a spinner that completes on success
pub fn with_spinner<F, T, E>(message: &str, complete_message: &str, func: F) -> Result<T, E>
where
    F: FnOnce() -> Result<T, E>,
{
    let spinner = spinner(message);
    let result = func();

    match &result {
        Ok(_) => {
            spinner.finish_with_message(format!("✓ {complete_message}"));
        }
        Err(_) => {
            spinner.finish_with_message(format!("✗ {message}"));
        }
    }

    result
}
