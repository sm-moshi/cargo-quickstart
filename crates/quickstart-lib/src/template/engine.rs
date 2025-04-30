//! Template engine implementation
//!
//! This module provides the core template rendering functionality
//! using Handlebars as the template engine.

use handlebars::{Context, Handlebars, Helper, HelperDef, Output, RenderContext, RenderError};

use super::{Result, TemplateError, TemplateVariables};

/// Template engine for rendering project templates
pub struct TemplateEngine {
    /// Handlebars registry
    handlebars: Handlebars<'static>,
    /// Template variables
    variables: TemplateVariables,
}

impl TemplateEngine {
    /// Create a new template engine with the given variables
    pub fn new(variables: TemplateVariables) -> Self {
        let mut handlebars = Handlebars::new();

        // Configure handlebars
        handlebars.set_strict_mode(true);

        // Register helpers
        handlebars.register_helper("lowercase", Box::new(LowercaseHelper));
        handlebars.register_helper("uppercase", Box::new(UppercaseHelper));
        handlebars.register_helper("snake_case", Box::new(SnakeCaseHelper));
        handlebars.register_helper("kebab_case", Box::new(KebabCaseHelper));

        Self {
            handlebars,
            variables,
        }
    }

    /// Register a template from a string
    pub fn register_template(&mut self, name: &str, content: &str) -> Result<()> {
        self.handlebars
            .register_template_string(name, content)
            .map_err(|e| TemplateError::RenderError {
                name: name.to_string(),
                source: e.into(),
            })
    }

    /// Render a template with the stored variables
    pub fn render(&self, template_name: &str) -> Result<String> {
        self.handlebars
            .render(template_name, &self.variables)
            .map_err(|e| TemplateError::RenderError {
                name: template_name.to_string(),
                source: e,
            })
    }

    /// Render a template string directly
    pub fn render_template(&self, template_content: &str) -> Result<String> {
        self.handlebars
            .render_template(template_content, &self.variables)
            .map_err(|e| TemplateError::RenderError {
                name: "string_template".to_string(),
                source: e,
            })
    }

    /// Get a reference to the template variables
    pub fn variables(&self) -> &TemplateVariables {
        &self.variables
    }

    /// Get a mutable reference to the template variables
    pub fn variables_mut(&mut self) -> &mut TemplateVariables {
        &mut self.variables
    }
}

/// Helper to convert text to lowercase
#[derive(Clone, Copy)]
struct LowercaseHelper;

impl HelperDef for LowercaseHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
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
struct UppercaseHelper;

impl HelperDef for UppercaseHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
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
struct SnakeCaseHelper;

impl HelperDef for SnakeCaseHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
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
struct KebabCaseHelper;

impl HelperDef for KebabCaseHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
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
fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let mut prev_is_sep = false;
    for (i, c) in input.char_indices() {
        if c.is_uppercase() {
            if i > 0 && !prev_is_sep {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
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
fn to_kebab_case(input: &str) -> String {
    let mut result = String::new();
    let mut prev_is_sep = false;
    for (i, c) in input.char_indices() {
        if c.is_uppercase() {
            if i > 0 && !prev_is_sep {
                result.push('-');
            }
            result.push(c.to_lowercase().next().unwrap());
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
    use crate::template::variables::TemplateVariables;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_template() {
        let variables = TemplateVariables::default_test_variables();
        let engine = TemplateEngine::new(variables);

        let template =
            "# {{name}}\n\n{{#if description}}{{description}}{{/if}}\n\nLicense: {{license}}";
        let expected = "# test-project\n\nA test project\n\nLicense: MIT";

        let result = engine.render_template(template).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_conditional_sections() {
        let mut variables = TemplateVariables::default_test_variables();
        variables.description = None;

        let engine = TemplateEngine::new(variables);

        let template = "# {{name}}\n\n{{#if description}}{{description}}{{else}}No description{{/if}}\n\nLicense: {{license}}";
        let expected = "# test-project\n\nNo description\n\nLicense: MIT";

        let result = engine.render_template(template).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_project_type_conditions() {
        let variables = TemplateVariables::default_test_variables();
        let engine = TemplateEngine::new(variables);

        let template = "{{#if project.is_binary}}Binary{{else}}Library{{/if}}";
        let expected = "Binary";

        let result = engine.render_template(template).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_lowercase_helper() {
        let variables = TemplateVariables::default_test_variables();
        let engine = TemplateEngine::new(variables);

        let template = "{{lowercase name}}";
        let expected = "test-project";

        let result = engine.render_template(template).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_uppercase_helper() {
        let variables = TemplateVariables::default_test_variables();
        let engine = TemplateEngine::new(variables);

        let template = "{{uppercase name}}";
        let expected = "TEST-PROJECT";

        let result = engine.render_template(template).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_snake_case_helper() {
        let variables = TemplateVariables::default_test_variables();
        let engine = TemplateEngine::new(variables);

        let template = "{{snake_case \"TestProject\"}}";
        let expected = "test_project";

        let result = engine.render_template(template).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_kebab_case_helper() {
        let variables = TemplateVariables::default_test_variables();
        let engine = TemplateEngine::new(variables);

        let template = "{{kebab_case \"test_project\"}}";
        let expected = "test-project";

        let result = engine.render_template(template).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_register_template_invalid_syntax() {
        let variables = TemplateVariables::default_test_variables();
        let mut engine = TemplateEngine::new(variables);
        // Invalid Handlebars syntax
        let result = engine.register_template("bad", "{{#if}");
        assert!(result.is_err());
    }

    #[test]
    fn test_render_unregistered_template() {
        let variables = TemplateVariables::default_test_variables();
        let engine = TemplateEngine::new(variables);
        // Try to render a template that was never registered
        let result = engine.render("not_registered");
        assert!(result.is_err());
    }

    #[test]
    fn test_render_template_invalid_content() {
        let variables = TemplateVariables::default_test_variables();
        let engine = TemplateEngine::new(variables);
        // Invalid Handlebars syntax in direct render
        let result = engine.render_template("{{#if}");
        assert!(result.is_err());
    }

    #[test]
    fn test_helper_missing_param() {
        let variables = TemplateVariables::default_test_variables();
        let engine = TemplateEngine::new(variables);
        // All helpers expect at least one param
        let helpers = ["lowercase", "uppercase", "snake_case", "kebab_case"];
        for helper in helpers {
            let template = format!("{{{{{} }}}}", helper); // No param
            let result = engine.render_template(&template);
            assert!(
                result.is_err(),
                "helper '{}' should error on missing param",
                helper
            );
        }
    }

    #[test]
    fn test_helper_non_string_param() {
        let variables = TemplateVariables::default_test_variables();
        let engine = TemplateEngine::new(variables);
        // Pass a number to helpers
        let helpers = ["lowercase", "uppercase", "snake_case", "kebab_case"];
        for helper in helpers {
            let template = format!("{{{{{} 123}}}}", helper);
            let result = engine.render_template(&template);
            // Should not panic, should handle gracefully (output empty string)
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "");
        }
    }

    #[test]
    fn test_variables_accessors() {
        let variables = TemplateVariables::default_test_variables();
        let mut engine = TemplateEngine::new(variables.clone());
        assert_eq!(engine.variables().name, variables.name);
        engine.variables_mut().name = "changed".to_string();
        assert_eq!(engine.variables().name, "changed");
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
