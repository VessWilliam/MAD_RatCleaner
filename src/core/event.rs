use crate::core::Action;
use crate::core::screen::Screen;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::io;
use std::time::Duration;

pub fn poll_action(timeout: Duration, screen: Screen) -> io::Result<Action> {
    if event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                return Ok(map_key(key.code, screen));
            }
        }
    }
    Ok(Action::None)
}

fn map_key(code: KeyCode, _screen: Screen) -> Action {
    match code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Enter => Action::Next,
        KeyCode::Esc | KeyCode::Backspace => Action::Prev,
        _ => Action::None,
    }
}
