//! Implementation of the 'completions' command for generating shell completions

use crate::args::{CompletionsArgs, Shell};
use clap::CommandFactory;
use clap_complete::{generate, Shell as ClapShell};
use std::fs::File;
use std::io::{self};

use crate::args::Cli;
use crate::ui::output;

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

    match &args.output {
        Some(path) => {
            let file = File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            generate(shell, &mut cmd, "cargo-quickstart", &mut writer);
            output::success(&format!(
                "Shell completions for {} written to {}",
                args.shell,
                path.display()
            ));
        }
        None => {
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
