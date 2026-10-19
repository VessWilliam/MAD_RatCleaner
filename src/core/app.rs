use ratatui::widgets::ListState;
use std::collections::HashMap;

use crate::core::screen::Screen;
use crate::core::trigger::Trigger;
use crate::states::{AppState, SCAN_LIST};
use crate::traits::{PluginGroup, StateStore};

type SystemFn = Box<dyn Fn(&mut App)>;

pub struct App {
    pub should_quit: bool,
    pub screen: Screen,
    pub state: AppState,
    systems: HashMap<Trigger, Vec<SystemFn>>,
}

impl App {
    pub fn new() -> Self {
        let mut state = AppState::new();
        let mut list = ListState::default();
        list.select(Some(0));
        state.register(SCAN_LIST, list);

        Self {
            should_quit: false,
            screen: Screen::Main,
            state,
            systems: HashMap::new(),
        }
    }

    pub fn add_plugins<G: PluginGroup>(mut self, group: G) -> Self {
        group.build_group(&mut self);
        self
    }

    pub fn add_system<F>(&mut self, trigger: Trigger, system: F) -> &mut Self
    where
        F: Fn(&mut App) + 'static,
    {
        self.systems
            .entry(trigger)
            .or_default()
            .push(Box::new(system));
        self
    }

    pub fn run_trigger(&mut self, trigger: Trigger) {
        let systems = self.systems.remove(&trigger).unwrap_or_default();
        for system in &systems {
            system(self)
        }
        self.systems.insert(trigger, systems);
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        crate::core::runtime::run(&mut self)
    }
}
