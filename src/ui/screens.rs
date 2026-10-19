use crate::{core::App, states::SCAN_LIST, traits::StateStore};
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
};

pub fn draw_main(frame: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = vec![
        ListItem::new("C:\\Windows\\Temp (1.2 GB)"),
        ListItem::new("npm cache (450 MB)"),
        ListItem::new("Cargo target (3.1 GB)"),
    ];

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Files to Clean ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Green)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    if let Some(state) = app.state.get_mut(SCAN_LIST) {
        frame.render_stateful_widget(list, area, state);
    }
}

pub fn draw_confirm(frame: &mut Frame, area: Rect) {
    let p = Paragraph::new("Are you sure?\n\nPress [Enter] to proceed.\nPress [Esc] to go back.")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(" Confirm ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Yellow)),
        );
    frame.render_widget(p, area);
}

pub fn draw_done(frame: &mut Frame, area: Rect) {
    let p =
        Paragraph::new("Cleanup Complete!\n\nPress [Enter] to restart.\nPress [Esc] to go back.")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(" Done ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::LightGreen)),
            );
    frame.render_widget(p, area);
}
