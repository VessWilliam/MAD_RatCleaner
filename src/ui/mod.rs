use std::rc::Rc;

use crate::{
    core::{App, Trigger},
    states::TERMINAL,
    traits::{Plugin, StateStore},
    ui::layout::draw,
};

// 1. Declare the struct file
pub mod footer;
pub mod header;
pub mod layout;
pub mod screens;

// 2. Define the Plugin right here!
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
