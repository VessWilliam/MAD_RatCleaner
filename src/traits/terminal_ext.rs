use ratatui::Frame;
use std::io::Result;

pub trait TerminalExt {
    fn draw(&mut self, render: &mut dyn FnMut(&mut Frame<'_>)) -> Result<()>;
}
