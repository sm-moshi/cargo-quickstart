//! Command implementations for cargo-quickstart

mod completions;
mod init;
mod new;

// pub use init::*;
// pub use new::*;

use crate::args::{CompletionsArgs, InitArgs, NewArgs};
use crate::ui::output;
use color_eyre::Result;

/// Execute the 'new' command
pub fn execute_new(args: NewArgs) -> Result<()> {
    new::execute(args)
}

/// Execute the 'init' command
pub fn execute_init(args: InitArgs) -> Result<()> {
    init::execute(args)
}

/// Execute the 'list-templates' command
pub fn execute_list_templates() -> Result<()> {
    use quickstart_lib::template::{TemplateLoader, TemplateVariant};
    use quickstart_lib::ProjectType;
    use std::collections::BTreeMap;

    // Find the templates directory
    let template_dir = quickstart_lib::find_templates_dir()?;
    let loader = TemplateLoader::new(template_dir);

    // Prepare a map to collect templates by type/variant
    let mut all_templates: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for &project_type in &[ProjectType::Binary, ProjectType::Library] {
        for &variant in &[TemplateVariant::Minimal, TemplateVariant::Extended] {
            let label = format!(
                "{:?}/{}",
                project_type,
                match variant {
                    TemplateVariant::Minimal => "minimal",
                    TemplateVariant::Extended => "extended",
                }
            );
            match loader.list_templates(project_type, variant) {
                Ok(templates) => {
                    let mut paths: Vec<String> = templates
                        .into_iter()
                        .map(|p| {
                            p.strip_prefix(loader.base_path())
                                .unwrap_or(&p)
                                .display()
                                .to_string()
                        })
                        .collect();
                    paths.sort();
                    all_templates.insert(label, paths);
                }
                Err(_) => {
                    // No templates for this type/variant, skip
                }
            }
        }
    }

    if all_templates.is_empty() {
        output::warning("No templates found.");
    } else {
        output::header("Available templates");
        for (label, templates) in all_templates {
            output::section(&label);
            for t in templates {
                output::list_item(&t);
            }
        }
    }
    Ok(())
}

/// Execute the 'completions' command
pub fn execute_completions(args: CompletionsArgs) -> color_eyre::Result<()> {
    completions::execute(args)
}
