use crate::core::App;
use crate::states::TERMINAL;
use crate::traits::{Plugin, StateStore, TerminalExt};
use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;

// 1. Declare the struct file
pub mod terminal;
use terminal::Terminal;

// 2. Define the Plugin right here!
pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        match Terminal::new() {
            Ok(term) => {
                let boxed: Rc<RefCell<Box<dyn TerminalExt>>> =
                    Rc::new(RefCell::new(Box::new(term)));
                app.state.register(TERMINAL, boxed);
            }
            Err(e) => {
                eprintln!("failed to init terminal: {e}");
                exit(1);
            }
        }
    }
}
