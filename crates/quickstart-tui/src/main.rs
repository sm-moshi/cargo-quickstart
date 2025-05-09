//! Binary entrypoint for manual TUI testing

use quickstart_tui::launch_tui;

fn main() {
    // Use colourful error reporting if available
    #[cfg(feature = "with-color-eyre")]
    {
        if let Err(e) = color_eyre::install() {
            eprintln!("Failed to install colour-eyre: {e}");
        }
    }

    match launch_tui() {
        Ok(()) => println!("\nExited TUI mode. Have a lovely day! 🐹"),
        Err(e) => eprintln!("TUI exited with an error: {e}"),
    }
}
