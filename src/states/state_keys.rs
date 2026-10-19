use crate::states::StateBase;
use crate::traits::TerminalExt;
use ratatui::widgets::ListState;
use std::cell::RefCell;
use std::rc::Rc;

pub const SCAN_LIST: StateBase<ListState> = StateBase::new("scan_list");
pub const TERMINAL: StateBase<Rc<RefCell<Box<dyn TerminalExt>>>> = StateBase::new("terminal");
