use super::{footer, header, screens};
use crate::App;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let [header_area, body_area, footer_area] = Layout::vertical([
        Constraint::Length(9),
        Constraint::Min(5),
        Constraint::Length(3),
    ])
    .areas(frame.area());

    header::draw(frame, header_area);

    match app.screen {
        crate::core::Screen::Main => screens::draw_main(frame, app, body_area),
        crate::core::Screen::Confirm => screens::draw_confirm(frame, body_area),
        crate::core::Screen::Done => screens::draw_done(frame, body_area),
    }

    footer::draw(frame, app, footer_area);
}
