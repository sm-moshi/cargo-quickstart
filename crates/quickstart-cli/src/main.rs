//! CLI entry point for cargo-quickstart

mod args;
mod commands;
mod errors;
mod ui;

use args::{Cli, Commands};
use clap::Parser;
use color_eyre::Result;

fn main() -> Result<()> {
    // Setup error handling
    errors::setup()?;

    // Parse command-line arguments
    let cli = Cli::parse();

    // Route to the appropriate command handler
    match cli.command {
        Commands::New(args) => commands::execute_new(args),
        Commands::Init(args) => commands::execute_init(args),
        Commands::ListTemplates => commands::execute_list_templates(),
        Commands::Completions(args) => commands::execute_completions(args),
    }
}
