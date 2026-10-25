use std::time::Duration;

use crate::core::action::Action;
use crate::core::app::App;
use crate::core::{Screen, Trigger, poll_action};

pub fn run(app: &mut App) -> anyhow::Result<()> {
    app.run_trigger(Trigger::Startup);

    while !app.should_quit {
        app.run_trigger(Trigger::Render);
        match poll_action(Duration::from_millis(100), app.screen)? {
            Action::None => {}
            action => {
                app.run_trigger(Trigger::On(action));
                handle_transitions(app, action);
            }
        }
    }
    Ok(())
}

fn handle_transitions(app: &mut App, action: Action) {
    match action {
        Action::Quit => app.should_quit = true,

        Action::Next => {
            app.screen = match app.screen {
                Screen::Main => Screen::Confirm,
                Screen::Confirm => Screen::Done,
                Screen::Done => Screen::Main,
            };
        }

        Action::Prev => {
            app.screen = match app.screen {
                Screen::Main => Screen::Main,
                Screen::Confirm => Screen::Main,
                Screen::Done => Screen::Confirm,
            };
        }

        _ => {}
    }
}
