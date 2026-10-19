use crate::core::App;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

pub fn draw(frame: &mut Frame, _app: &App, area: Rect) {
    let keys = map_key();

    let footer = Paragraph::new(keys)
        .alignment(Alignment::Center)
        .block(Block::default().border_type(BorderType::Rounded));
    frame.render_widget(footer, area);
}

fn map_key() -> Line<'static> {
    let keys = Line::from(vec![
        chip(" Enter ", Color::Green),
        Span::raw(" Next  "),
        chip(" Esc ", Color::Yellow),
        Span::raw(" Back  "),
        chip(" Q ", Color::Red),
        Span::raw(" Quit "),
    ]);
    keys
}

fn chip(text: &str, color: Color) -> Span<'static> {
    Span::styled(
        text.to_string(),
        Style::default().fg(Color::Black).bg(color),
    )
}
