use std::io::Result;
use std::io::{Stdout, stdout};

use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::terminal::disable_raw_mode;
use crossterm::{execute, terminal::enable_raw_mode};
use ratatui::Terminal as RatatuiTerminal;
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Frame;

use crate::traits::TerminalExt;

pub struct Terminal {
    inner: RatatuiTerminal<CrosstermBackend<Stdout>>,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let inner = RatatuiTerminal::new(backend)?;
        Ok(Self { inner })
    }
}

impl TerminalExt for Terminal {
    fn draw(&mut self, render: &mut dyn FnMut(&mut Frame<'_>)) -> Result<()> {
        self.inner.draw(|frame| render(frame)).map(|_| ())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.inner.backend_mut(), LeaveAlternateScreen);
        let _ = self.inner.show_cursor();
    }
}
