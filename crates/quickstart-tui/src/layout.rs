//! Layout and rendering components for cargo-quickstart TUI

use crate::app_state::TuiState;
use color_eyre::Result;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use ratatui::Terminal;
use std::io;

/// Render the current TUI state to the frame.
pub fn render_tui_state(frame: &mut Frame, state: &TuiState) {
    match state {
        TuiState::Input { project_name } => render_input_screen(frame, project_name),
        TuiState::Confirm { project_name } => render_confirm_screen(frame, project_name),
        TuiState::Done => render_done_screen(frame),
    }
}

fn render_input_screen(frame: &mut Frame, project_name: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(frame.area());

    let title = Paragraph::new("Enter project name:")
        .block(Block::default().borders(Borders::ALL).title("Input"));
    frame.render_widget(title, chunks[0]);

    let input = Paragraph::new(project_name)
        .block(Block::default().borders(Borders::ALL).title("Project Name"));
    frame.render_widget(input, chunks[1]);
    // TODO: Add cursor rendering if desired
}

fn render_confirm_screen(frame: &mut Frame, project_name: &str) {
    let text =
        format!("Project Name: {project_name}\n\nPress 'y' to confirm, 'n' to edit, 'q' to quit.");
    let paragraph =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Confirm"));
    frame.render_widget(paragraph, frame.area());
}

fn render_done_screen(frame: &mut Frame) {
    let paragraph = Paragraph::new("Project generation complete! Press 'q' to quit.")
        .block(Block::default().borders(Borders::ALL).title("Done"));
    frame.render_widget(paragraph, frame.area());
}

pub fn run_tui(initial_project_name: Option<String>, dry_run: bool) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = TuiState::Input {
        project_name: initial_project_name.unwrap_or_default(),
    };

    // Main event loop
    loop {
        terminal.draw(|f| render_tui_state(f, &app_state))?;

        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
                if key_event.kind == crossterm::event::KeyEventKind::Press {
                    match app_state {
                        TuiState::Input {
                            ref mut project_name,
                        } => match key_event.code {
                            crossterm::event::KeyCode::Enter => {
                                app_state = app_state.to_confirm();
                            }
                            crossterm::event::KeyCode::Char(c) => {
                                project_name.push(c);
                            }
                            crossterm::event::KeyCode::Backspace => {
                                project_name.pop();
                            }
                            crossterm::event::KeyCode::Esc => {
                                // TODO: Decide if Esc should quit or go back. For now, quit.
                                break;
                            }
                            _ => {}
                        },
                        TuiState::Confirm { .. } => match key_event.code {
                            crossterm::event::KeyCode::Char('y')
                            | crossterm::event::KeyCode::Char('Y') => {
                                // TODO: Trigger project generation with quickstart_lib::generate_project()
                                // For now, just transition to Done state.
                                println!(
                                    "Project generation would happen here for: {}. Dry run: {}",
                                    match &app_state {
                                        TuiState::Confirm { project_name } => project_name,
                                        _ => "ERROR_STATE", // Should not happen
                                    },
                                    dry_run
                                );
                                app_state = app_state.to_done();
                            }
                            crossterm::event::KeyCode::Char('n')
                            | crossterm::event::KeyCode::Char('N') => {
                                app_state = app_state.to_input();
                            }
                            crossterm::event::KeyCode::Char('q')
                            | crossterm::event::KeyCode::Char('Q')
                            | crossterm::event::KeyCode::Esc => {
                                break;
                            }
                            _ => {}
                        },
                        TuiState::Done => match key_event.code {
                            crossterm::event::KeyCode::Char('q')
                            | crossterm::event::KeyCode::Char('Q')
                            | crossterm::event::KeyCode::Esc => {
                                break;
                            }
                            _ => {}
                        },
                    }
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;

    fn render_to_buffer(
        state: &TuiState,
        width: u16,
        height: u16,
    ) -> Result<Buffer, color_eyre::Report> {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend)?;
        terminal.draw(|f| render_tui_state(f, state))?;
        Ok(terminal.backend().buffer().clone())
    }

    // Helper to check if buffer's string content contains text
    // This is a simplified check; more robust checks might inspect cell attributes.
    fn buffer_contains_text(buffer: &Buffer, text: &str) -> bool {
        buffer
            .content
            .iter()
            .map(|cell| cell.symbol())
            .collect::<String>()
            .contains(text)
    }

    #[test]
    fn test_render_input_screen() -> Result<(), color_eyre::Report> {
        let state = TuiState::Input {
            project_name: "test-project".to_string(),
        };
        let buffer = render_to_buffer(&state, 60, 10)?;
        assert!(buffer_contains_text(&buffer, "Enter project name:"));
        assert!(buffer_contains_text(&buffer, "test-project"));
        Ok(())
    }

    #[test]
    fn test_render_confirm_screen() -> Result<(), color_eyre::Report> {
        let state = TuiState::Confirm {
            project_name: "my-app".to_string(),
        };
        let buffer = render_to_buffer(&state, 60, 10)?;
        assert!(buffer_contains_text(&buffer, "Project Name: my-app"));
        assert!(buffer_contains_text(&buffer, "Press 'y' to confirm"));
        Ok(())
    }

    #[test]
    fn test_render_done_screen() -> Result<(), color_eyre::Report> {
        let state = TuiState::Done;
        let buffer = render_to_buffer(&state, 60, 10)?;
        assert!(buffer_contains_text(
            &buffer,
            "Project generation complete!"
        ));
        Ok(())
    }
}

// TODO: Implement TUI layout (input, confirmation, config printout) - This is partially addressed now.
