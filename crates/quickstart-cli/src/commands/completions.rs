//! Implementation of the 'completions' command for generating shell completions

use crate::args::{CompletionsArgs, Shell};
use clap::CommandFactory;
use clap_complete::{generate, Shell as ClapShell};
use color_eyre::eyre::Context;
use std::fs::File;
use std::io::{self};
use std::path::PathBuf;

use crate::args::Cli;
use crate::ui::output;

/// The destination for shell completions output
enum CompletionOutput {
    /// Write to a file at the specified path
    File(PathBuf),
    /// Write to standard output
    Stdout,
}

/// Execute the 'completions' command
pub fn execute(args: CompletionsArgs) -> color_eyre::Result<()> {
    let mut cmd = Cli::command();
    let shell = match args.shell {
        Shell::Bash => ClapShell::Bash,
        Shell::Zsh => ClapShell::Zsh,
        Shell::Fish => ClapShell::Fish,
        Shell::Powershell => ClapShell::PowerShell,
        Shell::Elvish => ClapShell::Elvish,
    };

    // Convert the Option<PathBuf> to our new enum
    let output = if let Some(path) = args.output {
        CompletionOutput::File(path)
    } else {
        CompletionOutput::Stdout
    };

    match output {
        CompletionOutput::File(path) => {
            // Enhanced error context for file operations
            let file = File::create(&path).with_context(|| {
                format!("Failed to create completions file at {}", path.display())
            })?;

            let mut writer = io::BufWriter::new(file);
            generate(shell, &mut cmd, "cargo-quickstart", &mut writer);

            output::success(&format!(
                "Shell completions for {} written to {}",
                args.shell,
                path.display()
            ));
        }
        CompletionOutput::Stdout => {
            let mut stdout = io::stdout();
            generate(shell, &mut cmd, "cargo-quickstart", &mut stdout);

            output::info(&format!(
                "Shell completions for {} written to stdout",
                args.shell
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::{CompletionsArgs, Shell};
    use color_eyre::Result;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_execute_writes_completions_to_file() -> Result<()> {
        // Skip this test when running under Miri
        if cfg!(miri) {
            eprintln!("Skipping file I/O test under Miri");
            return Ok(());
        }

        for shell in [
            Shell::Bash,
            Shell::Zsh,
            Shell::Fish,
            Shell::Powershell,
            Shell::Elvish,
        ] {
            let tmp = NamedTempFile::new().with_context(|| {
                format!("Failed to create temporary file for {shell:?} completions")
            })?;
            let path = tmp.path().to_path_buf();
            let args = CompletionsArgs {
                shell: shell.clone(),
                output: Some(path.clone()),
            };
            let result = execute(args);
            assert!(result.is_ok(), "execute() should succeed for {shell:?}");
            let contents = fs::read(&path)
                .with_context(|| format!("Failed to read completions file for {shell:?}"))?;
            assert!(
                !contents.is_empty(),
                "Completions file should not be empty for {shell:?}"
            );
        }
        Ok(())
    }

    #[test]
    fn test_execute_writes_completions_to_stdout() -> Result<()> {
        // Skip this test when running under Miri
        if cfg!(miri) {
            eprintln!("Skipping stdout I/O test under Miri");
            return Ok(());
        }

        for shell in [
            Shell::Bash,
            Shell::Zsh,
            Shell::Fish,
            Shell::Powershell,
            Shell::Elvish,
        ] {
            let args = CompletionsArgs {
                shell: shell.clone(),
                output: None,
            };
            let result = execute(args);
            assert!(
                result.is_ok(),
                "execute() should succeed for {shell:?} to stdout"
            );
        }
        Ok(())
    }
}
