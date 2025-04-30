//! Command implementations for cargo-quickstart

mod init;
mod new;

// pub use init::*;
// pub use new::*;

use crate::args::{InitArgs, NewArgs};
use color_eyre::Result;

/// Execute the 'new' command
pub fn execute_new(args: NewArgs) -> Result<()> {
    new::execute(args)
}

/// Execute the 'init' command
pub fn execute_init(args: InitArgs) -> Result<()> {
    init::execute(args)
}
