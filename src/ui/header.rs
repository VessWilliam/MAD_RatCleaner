use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn draw(frame: &mut Frame, area: Rect) {
    let title = format!("---  Mad Rat UI v{} ---", env!("CARGO_PKG_VERSION"));
    let text = logo();

    let header = Paragraph::new(text)
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(title)
                .title_alignment(Alignment::Center)
                .title_style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Green)),
        );
    frame.render_widget(header, area);
}

fn logo() -> Text<'static> {
    Text::from(vec![
        Line::from("---------------------"),
        Line::from("   q_ p_ "),
        Line::from("   /   #\\"),
        Line::from(" (    )"),
        Line::from("  `-(-'"),
        Line::from("------------)--------"),
    ])
}
