use std::any::Any;
use std::collections::HashMap;

use crate::states::StateBase;
use crate::traits::StateStore;

#[derive(Default)]
pub struct AppState {
    store: HashMap<&'static str, Box<dyn Any>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl StateStore for AppState {
    fn register<T: 'static>(&mut self, key: StateBase<T>, value: T) {
        self.store.insert(key.name, Box::new(value));
    }
    fn get<T: 'static>(&self, key: StateBase<T>) -> Option<&T> {
        self.store.get(key.name)?.downcast_ref::<T>()
    }
    fn get_mut<T: 'static>(&mut self, key: StateBase<T>) -> Option<&mut T> {
        self.store.get_mut(key.name)?.downcast_mut::<T>()
    }
}
