pub mod footer;
pub mod header;
pub mod main;
pub mod screens;

use std::rc::Rc;

use crate::{
    core::{App, Trigger},
    states::TERMINAL,
    traits::{Plugin, StateStore},
    ui::main::draw,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Trigger::Render, |app| {
            let terminal = {
                let term = app.state.get(TERMINAL).expect("Terminal not init");
                Rc::clone(term)
            };

            let _ = terminal.borrow_mut().draw(&mut |frame| draw(frame, app));
        });
    }
}
