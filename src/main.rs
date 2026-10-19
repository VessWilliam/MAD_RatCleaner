use crate::core::App;
use crate::terminal::TerminalPlugin;
use crate::ui::UiPlugin;

mod core;
mod states;
mod terminal;
mod traits;
mod ui;

fn main() -> anyhow::Result<()> {
    App::new().add_plugins((TerminalPlugin, UiPlugin)).run()
}
