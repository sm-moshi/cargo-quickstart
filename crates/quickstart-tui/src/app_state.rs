//! State management for cargo-quickstart TUI

/// Represents the current state of the TUI application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TuiState {
    /// User is entering the project name.
    Input { project_name: String },
    /// User is confirming the project name.
    Confirm { project_name: String },
    /// Project creation is done.
    Done,
}

impl TuiState {
    /// Transition from input to confirm state.
    pub fn to_confirm(&self) -> Self {
        match self {
            TuiState::Input { project_name } => TuiState::Confirm {
                project_name: project_name.clone(),
            },
            _ => self.clone(),
        }
    }

    /// Transition from confirm to input state (edit).
    pub fn to_input(&self) -> Self {
        match self {
            TuiState::Confirm { project_name } => TuiState::Input {
                project_name: project_name.clone(),
            },
            _ => self.clone(),
        }
    }

    /// Transition to done state.
    pub fn to_done(&self) -> Self {
        TuiState::Done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_to_confirm_transitions_correctly() {
        let state = TuiState::Input {
            project_name: "foo".to_string(),
        };
        let next = state.to_confirm();
        assert_eq!(
            next,
            TuiState::Confirm {
                project_name: "foo".to_string()
            }
        );
    }

    #[test]
    fn confirm_to_input_transitions_correctly() {
        let state = TuiState::Confirm {
            project_name: "bar".to_string(),
        };
        let next = state.to_input();
        assert_eq!(
            next,
            TuiState::Input {
                project_name: "bar".to_string()
            }
        );
    }

    #[test]
    fn any_state_to_done_transitions_correctly() {
        let input = TuiState::Input {
            project_name: "baz".to_string(),
        };
        let confirm = TuiState::Confirm {
            project_name: "baz".to_string(),
        };
        let done = TuiState::Done;
        assert_eq!(input.to_done(), TuiState::Done);
        assert_eq!(confirm.to_done(), TuiState::Done);
        assert_eq!(done.to_done(), TuiState::Done);
    }

    #[test]
    fn debug_and_clone_are_implemented() {
        let state = TuiState::Input {
            project_name: "clone".to_string(),
        };
        let clone = state.clone();
        assert_eq!(state, clone);
        let debug_str = format!("{state:?}");
        assert!(debug_str.contains("Input"));
    }

    #[test]
    fn non_input_state_to_confirm_returns_self() {
        let done_state = TuiState::Done;
        let result = done_state.to_confirm();
        assert_eq!(result, done_state);

        let confirm_state = TuiState::Confirm {
            project_name: "test".to_string(),
        };
        let result = confirm_state.to_confirm();
        assert_eq!(result, confirm_state);
    }

    #[test]
    fn non_confirm_state_to_input_returns_self() {
        let done_state = TuiState::Done;
        let result = done_state.to_input();
        assert_eq!(result, done_state);

        let input_state = TuiState::Input {
            project_name: "test".to_string(),
        };
        let result = input_state.to_input();
        assert_eq!(result, input_state);
    }
}

// TODO: Add more state logic as TUI grows
