//! Event handling for cargo-quickstart TUI

use crate::app_state::TuiState;
use crossterm::event::{Event, KeyCode, KeyEventKind};

/// Handle a single event and return the new state.
pub fn handle_event(state: &TuiState, event: &Event) -> TuiState {
    match state {
        TuiState::Input { project_name } => handle_input_event(project_name, event),
        TuiState::Confirm { project_name } => handle_confirm_event(project_name, event),
        TuiState::Done => handle_done_event(event),
    }
}

fn handle_input_event(project_name: &str, event: &Event) -> TuiState {
    let mut name = project_name.to_string();
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char(c) if key.kind == KeyEventKind::Press => {
                name.push(c);
                TuiState::Input { project_name: name }
            }
            KeyCode::Backspace if key.kind == KeyEventKind::Press => {
                name.pop();
                TuiState::Input { project_name: name }
            }
            KeyCode::Enter if key.kind == KeyEventKind::Press => {
                TuiState::Confirm { project_name: name }
            }
            KeyCode::Esc if key.kind == KeyEventKind::Press => TuiState::Done,
            _ => TuiState::Input { project_name: name },
        }
    } else {
        TuiState::Input { project_name: name }
    }
}

fn handle_confirm_event(project_name: &str, event: &Event) -> TuiState {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('y') if key.kind == KeyEventKind::Press => TuiState::Done,
            KeyCode::Char('n') if key.kind == KeyEventKind::Press => TuiState::Input {
                project_name: project_name.to_string(),
            },
            KeyCode::Esc if key.kind == KeyEventKind::Press => TuiState::Done,
            _ => TuiState::Confirm {
                project_name: project_name.to_string(),
            },
        }
    } else {
        TuiState::Confirm {
            project_name: project_name.to_string(),
        }
    }
}

fn handle_done_event(event: &Event) -> TuiState {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('q') if key.kind == KeyEventKind::Press => TuiState::Done,
            _ => TuiState::Done,
        }
    } else {
        TuiState::Done
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEvent, KeyEventState, KeyModifiers};

    // Helper to create a KeyEvent with sensible defaults
    fn create_key_event(code: KeyCode) -> Event {
        Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
            modifiers: KeyModifiers::NONE,
        })
    }

    #[test]
    fn input_state_char_pushes_to_name() {
        let state = TuiState::Input {
            project_name: "foo".to_string(),
        };
        let event = create_key_event(KeyCode::Char('a'));
        let next = handle_event(&state, &event);
        assert_eq!(
            next,
            TuiState::Input {
                project_name: "fooa".to_string()
            }
        );
    }

    #[test]
    fn input_state_backspace_removes_char() {
        let state = TuiState::Input {
            project_name: "bar".to_string(),
        };
        let event = create_key_event(KeyCode::Backspace);
        let next = handle_event(&state, &event);
        assert_eq!(
            next,
            TuiState::Input {
                project_name: "ba".to_string()
            }
        );
    }

    #[test]
    fn input_state_enter_moves_to_confirm() {
        let state = TuiState::Input {
            project_name: "baz".to_string(),
        };
        let event = create_key_event(KeyCode::Enter);
        let next = handle_event(&state, &event);
        assert_eq!(
            next,
            TuiState::Confirm {
                project_name: "baz".to_string()
            }
        );
    }

    #[test]
    fn input_state_esc_moves_to_done() {
        let state = TuiState::Input {
            project_name: "baz".to_string(),
        };
        let event = create_key_event(KeyCode::Esc);
        let next = handle_event(&state, &event);
        assert_eq!(next, TuiState::Done);
    }

    #[test]
    fn confirm_state_y_moves_to_done() {
        let state = TuiState::Confirm {
            project_name: "foo".to_string(),
        };
        let event = create_key_event(KeyCode::Char('y'));
        let next = handle_event(&state, &event);
        assert_eq!(next, TuiState::Done);
    }

    #[test]
    fn confirm_state_n_moves_to_input() {
        let state = TuiState::Confirm {
            project_name: "foo".to_string(),
        };
        let event = create_key_event(KeyCode::Char('n'));
        let next = handle_event(&state, &event);
        assert_eq!(
            next,
            TuiState::Input {
                project_name: "foo".to_string()
            }
        );
    }

    #[test]
    fn confirm_state_esc_moves_to_done() {
        let state = TuiState::Confirm {
            project_name: "foo".to_string(),
        };
        let event = create_key_event(KeyCode::Esc);
        let next = handle_event(&state, &event);
        assert_eq!(next, TuiState::Done);
    }

    #[test]
    fn done_state_q_keeps_done() {
        let state = TuiState::Done;
        let event = create_key_event(KeyCode::Char('q'));
        let next = handle_event(&state, &event);
        assert_eq!(next, TuiState::Done);
    }

    #[test]
    fn done_state_other_key_keeps_done() {
        let state = TuiState::Done;
        let event = create_key_event(KeyCode::Char('x'));
        let next = handle_event(&state, &event);
        assert_eq!(next, TuiState::Done);
    }
}

// TODO: Implement terminal input and event loop
