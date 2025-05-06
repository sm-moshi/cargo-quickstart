//! Template engine implementation
//!
//! This module provides the core template rendering functionality
//! using Handlebars as the template engine.

use handlebars::Handlebars;

use super::{Result, TemplateError, TemplateVariables};
use crate::tools::{KebabCaseHelper, LowercaseHelper, SnakeCaseHelper, UppercaseHelper};

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
    fn test_variables_accessors() {
        let variables = TemplateVariables::default_test_variables();
        let mut engine = TemplateEngine::new(variables.clone());
        assert_eq!(engine.variables().name, variables.name);
        engine.variables_mut().name = "changed".to_string();
        assert_eq!(engine.variables().name, "changed");
    }
}
