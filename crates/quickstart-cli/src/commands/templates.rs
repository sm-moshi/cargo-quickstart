//! Template management functionality

use color_eyre::Result;
use quickstart_lib::{template::TemplateVariant, ProjectType};
use std::collections::BTreeMap;

use crate::ui::output;

/// Collect templates from all project types and variants
pub fn collect_templates(
    loader: &quickstart_lib::template::TemplateLoader,
) -> Result<BTreeMap<String, Vec<String>>> {
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
                                .map(|path| path.display().to_string())
                                .unwrap_or_else(|_| p.display().to_string())
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

    Ok(all_templates)
}

/// Display collected templates in a structured format
pub fn display_templates(all_templates: BTreeMap<String, Vec<String>>) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_templates_empty() -> Result<()> {
        let temp_dir = tempfile::TempDir::new()?;
        let loader = quickstart_lib::template::TemplateLoader::new(temp_dir.path());
        let templates = collect_templates(&loader)?;
        assert!(templates.is_empty());
        Ok(())
    }

    #[test]
    fn test_collect_templates_with_content() -> Result<()> {
        let temp_dir = tempfile::TempDir::new()?;
        let binary_dir = temp_dir.path().join("binary").join("minimal");
        std::fs::create_dir_all(&binary_dir)?;
        std::fs::write(binary_dir.join("test.hbs"), "test content")?;

        let loader = quickstart_lib::template::TemplateLoader::new(temp_dir.path());
        let templates = collect_templates(&loader)?;
        assert!(!templates.is_empty());
        Ok(())
    }
}
