use crate::core::action::Action;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Trigger {
    Startup,
    Render,
    On(Action),
}
