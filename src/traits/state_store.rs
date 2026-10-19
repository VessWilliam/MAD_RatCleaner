use crate::states::StateBase;

pub trait StateStore {
    fn register<T: 'static>(&mut self, key: StateBase<T>, value: T);
    fn get<T: 'static>(&self, key: StateBase<T>) -> Option<&T>;
    fn get_mut<T: 'static>(&mut self, key: StateBase<T>) -> Option<&mut T>;
}
