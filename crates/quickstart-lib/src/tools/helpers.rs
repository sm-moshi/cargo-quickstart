//! Handlebars template helpers
//!
//! This module provides custom helpers for use in handlebars templates,
//! such as text case conversion utilities.

use handlebars::{Context, Helper, HelperDef, Output, RenderContext, RenderError};
use thiserror::Error;

#[allow(dead_code)]
/// Error type for helpers
#[derive(Debug, Error)]
pub enum HelperError {
    /// Error when lowercase character conversion fails
    #[error("Failed to convert character to lowercase")]
    LowercaseConversionError,
}

/// Helper to convert text to lowercase
#[derive(Clone, Copy)]
pub struct LowercaseHelper;

impl HelperDef for LowercaseHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg handlebars::Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> handlebars::HelperResult {
        let param = h.param(0).ok_or_else(|| {
            RenderError::from(handlebars::RenderErrorReason::ParamNotFoundForIndex(
                "lowercase",
                0,
            ))
        })?;

        let value = param.value().as_str().unwrap_or_default().to_lowercase();
        out.write(&value)?;

        Ok(())
    }
}

/// Helper to convert text to uppercase
#[derive(Clone, Copy)]
pub struct UppercaseHelper;

impl HelperDef for UppercaseHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg handlebars::Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> handlebars::HelperResult {
        let param = h.param(0).ok_or_else(|| {
            RenderError::from(handlebars::RenderErrorReason::ParamNotFoundForIndex(
                "uppercase",
                0,
            ))
        })?;

        let value = param.value().as_str().unwrap_or_default().to_uppercase();
        out.write(&value)?;

        Ok(())
    }
}

/// Helper to convert text to snake_case
#[derive(Clone, Copy)]
pub struct SnakeCaseHelper;

impl HelperDef for SnakeCaseHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg handlebars::Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> handlebars::HelperResult {
        let param = h.param(0).ok_or_else(|| {
            RenderError::from(handlebars::RenderErrorReason::ParamNotFoundForIndex(
                "snake_case",
                0,
            ))
        })?;

        let input = param.value().as_str().unwrap_or_default();
        let value = to_snake_case(input);
        out.write(&value)?;

        Ok(())
    }
}

/// Helper to convert text to kebab-case
#[derive(Clone, Copy)]
pub struct KebabCaseHelper;

impl HelperDef for KebabCaseHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg handlebars::Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> handlebars::HelperResult {
        let param = h.param(0).ok_or_else(|| {
            RenderError::from(handlebars::RenderErrorReason::ParamNotFoundForIndex(
                "kebab_case",
                0,
            ))
        })?;

        let input = param.value().as_str().unwrap_or_default();
        let value = to_kebab_case(input);
        out.write(&value)?;

        Ok(())
    }
}

/// Convert a string to snake_case
pub fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let mut prev_is_sep = false;
    for (i, c) in input.char_indices() {
        if c.is_uppercase() {
            if i > 0 && !prev_is_sep {
                result.push('_');
            }
            if let Some(lc) = c.to_lowercase().next() {
                result.push(lc);
            }
            prev_is_sep = false;
        } else if c == '-' || c == ' ' || c == '_' {
            if !prev_is_sep {
                result.push('_');
                prev_is_sep = true;
            }
        } else {
            result.push(c);
            prev_is_sep = false;
        }
    }
    result
}

/// Convert a string to kebab-case
pub fn to_kebab_case(input: &str) -> String {
    let mut result = String::new();
    let mut prev_is_sep = false;
    for (i, c) in input.char_indices() {
        if c.is_uppercase() {
            if i > 0 && !prev_is_sep {
                result.push('-');
            }
            if let Some(lc) = c.to_lowercase().next() {
                result.push(lc);
            }
            prev_is_sep = false;
        } else if c == '_' || c == ' ' || c == '-' {
            if !prev_is_sep {
                result.push('-');
                prev_is_sep = true;
            }
        } else {
            result.push(c);
            prev_is_sep = false;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use handlebars::Handlebars;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_lowercase_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("lowercase", Box::new(LowercaseHelper));

        let template = "{{lowercase value}}";
        let mut data = std::collections::HashMap::new();
        data.insert("value", "TEST-PROJECT");

        let result = handlebars.render_template(template, &data).unwrap();
        assert_eq!(result, "test-project");
    }

    #[test]
    fn test_uppercase_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("uppercase", Box::new(UppercaseHelper));

        let template = "{{uppercase value}}";
        let mut data = std::collections::HashMap::new();
        data.insert("value", "test-project");

        let result = handlebars.render_template(template, &data).unwrap();
        assert_eq!(result, "TEST-PROJECT");
    }

    #[test]
    fn test_snake_case_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("snake_case", Box::new(SnakeCaseHelper));

        let template = "{{snake_case value}}";
        let mut data = std::collections::HashMap::new();
        data.insert("value", "TestProject");

        let result = handlebars.render_template(template, &data).unwrap();
        assert_eq!(result, "test_project");
    }

    #[test]
    fn test_kebab_case_helper() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("kebab_case", Box::new(KebabCaseHelper));

        let template = "{{kebab_case value}}";
        let mut data = std::collections::HashMap::new();
        data.insert("value", "test_project");

        let result = handlebars.render_template(template, &data).unwrap();
        assert_eq!(result, "test-project");
    }

    #[test]
    fn test_helper_missing_param() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("lowercase", Box::new(LowercaseHelper));
        handlebars.register_helper("uppercase", Box::new(UppercaseHelper));
        handlebars.register_helper("snake_case", Box::new(SnakeCaseHelper));
        handlebars.register_helper("kebab_case", Box::new(KebabCaseHelper));

        let helpers = ["lowercase", "uppercase", "snake_case", "kebab_case"];
        for helper in helpers {
            let template = format!("{{{{{helper}}}}}"); // No param
            let result = handlebars
                .render_template(&template, &std::collections::HashMap::<&str, &str>::new());
            assert!(
                result.is_err(),
                "helper '{helper}' should error on missing param"
            );
        }
    }

    #[test]
    fn test_helper_non_string_param() {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("lowercase", Box::new(LowercaseHelper));
        handlebars.register_helper("uppercase", Box::new(UppercaseHelper));
        handlebars.register_helper("snake_case", Box::new(SnakeCaseHelper));
        handlebars.register_helper("kebab_case", Box::new(KebabCaseHelper));

        let mut data = std::collections::HashMap::new();
        data.insert("value", 123);

        let helpers = ["lowercase", "uppercase", "snake_case", "kebab_case"];
        for helper in helpers {
            let template = format!("{{{{{helper} value}}}}");
            let result = handlebars.render_template(&template, &data);
            // Should not panic, should handle gracefully (output empty string)
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "");
        }
    }

    #[test]
    fn test_to_snake_case_edge_cases() {
        assert_eq!(to_snake_case("").as_str(), "");
        assert_eq!(to_snake_case("Already_Snake").as_str(), "already_snake");
        assert_eq!(to_snake_case("with-dash").as_str(), "with_dash");
        assert_eq!(to_snake_case("with space").as_str(), "with_space");
        assert_eq!(to_snake_case("123ABC").as_str(), "123_a_b_c");
    }

    #[test]
    fn test_to_kebab_case_edge_cases() {
        assert_eq!(to_kebab_case("").as_str(), "");
        assert_eq!(to_kebab_case("Already-Kebab").as_str(), "already-kebab");
        assert_eq!(to_kebab_case("with_underscore").as_str(), "with-underscore");
        assert_eq!(to_kebab_case("with space").as_str(), "with-space");
        assert_eq!(to_kebab_case("123ABC").as_str(), "123-a-b-c");
    }
}
