use crate::app::App;
use crossterm::event::{self, Event, KeyCode};
use std::error::Error;

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_events(&self, app: &mut App) -> Result<bool, Box<dyn Error>> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => {
                    app.add_task();
                }
                KeyCode::Delete => {
                    app.delete_task();
                }
                KeyCode::Char(c) => {
                    if c == 'q' {
                        return Ok(true);
                    }
                    app.input.push(c);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                KeyCode::Up => app.move_selection_up(),
                KeyCode::Down => app.move_selection_down(),
                _ => {}
            }
        }
        Ok(false)
    }
}
