# Test Fixtures

This directory contains test fixtures for integration testing of the CLI. These files are used to test various aspects of the application, particularly interactive prompts.

## Fixture Types

### Input Validation

-   `valid_project_name.txt` - Contains a valid project name for testing the project name validator
-   `invalid_project_name_*.txt` - Contains various invalid project names for negative testing

### Configuration Testing

-   `default_config.txt` - Default configuration for testing with default values
-   `custom_config.txt` - Custom configuration for testing with user-provided values

## Usage

These fixtures can be used in integration tests by redirecting their content as stdin:

```rust
// Example usage in a test
let input = include_str!("fixtures/valid_project_name.txt");
let mut cmd = Command::cargo_bin("cargo-quickstart")
    .arg("init")
    .write_stdin(input)
    .assert()
    .success();
```
