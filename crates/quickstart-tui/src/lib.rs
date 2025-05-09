//! TUI entrypoint for cargo-quickstart

pub mod app_state;
pub mod events;
pub mod layout;
pub use layout::run_tui;

use app_state::TuiState;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use events::handle_event;
use layout::render_tui_state;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{stdout, Result};

/// Launch the TUI
pub fn launch_tui() -> Result<()> {
    // Enable raw mode and enter alternate screen
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // Set initial state
    let mut state = TuiState::Input {
        project_name: String::new(),
    };

    loop {
        // Render current state
        terminal.draw(|frame| render_tui_state(frame, &state))?;

        // Handle input events
        if let Event::Key(key) = event::read()? {
            // Update state based on event
            state = handle_event(&state, &Event::Key(key));

            // Exit if we're done and the user pressed 'q'
            if matches!(state, TuiState::Done) && key.code == crossterm::event::KeyCode::Char('q') {
                break;
            }
        }
    }

    // Restore terminal state before exit
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify the state transitions within the TUI flow by simulating events
    #[test]
    fn test_tui_state_transitions() {
        // Initial state should be Input with empty project name
        let state = TuiState::Input {
            project_name: String::new(),
        };

        // Typing should add characters to project name
        let char_a_event = Event::Key(crossterm::event::KeyEvent {
            code: crossterm::event::KeyCode::Char('a'),
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
            modifiers: crossterm::event::KeyModifiers::NONE,
        });

        let state_after_typing = handle_event(&state, &char_a_event);
        assert!(
            matches!(state_after_typing, TuiState::Input { ref project_name } if project_name == "a")
        );

        // Pressing Enter should transition to Confirm
        let enter_event = Event::Key(crossterm::event::KeyEvent {
            code: crossterm::event::KeyCode::Enter,
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
            modifiers: crossterm::event::KeyModifiers::NONE,
        });

        let state_after_enter = handle_event(&state_after_typing, &enter_event);
        assert!(
            matches!(state_after_enter, TuiState::Confirm { ref project_name } if project_name == "a")
        );

        // Pressing Y should complete the flow
        let y_event = Event::Key(crossterm::event::KeyEvent {
            code: crossterm::event::KeyCode::Char('y'),
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
            modifiers: crossterm::event::KeyModifiers::NONE,
        });

        let state_after_y = handle_event(&state_after_enter, &y_event);
        assert!(matches!(state_after_y, TuiState::Done));
    }
}
