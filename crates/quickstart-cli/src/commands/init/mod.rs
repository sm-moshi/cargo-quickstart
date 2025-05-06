//! Implementation of the 'init' command for initializing a project in an existing directory

mod config;
mod executor;
mod inquire_api;
mod interactive;
mod tests;

use crate::args::InitArgs;
use crate::errors::CommandErrorExt;
use crate::ui::progress::with_spinner;
use color_eyre::Result;
use quickstart_lib::generate_project;

/// Execute the init command
pub fn execute(args: InitArgs) -> Result<()> {
    if args.interactive {
        // Use the enhanced interactive mode
        let config = interactive::run_wizard(args.path.clone())?;
        with_spinner(
            "Initializing project...",
            "Project initialized successfully!",
            || generate_project(config).command_context("init"),
        )
    } else {
        // Use the standard execution path
        executor::execute(args)
    }
}
