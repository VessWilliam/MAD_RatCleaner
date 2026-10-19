use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct StateBase<T> {
    pub name: &'static str,
    _maker: PhantomData<fn() -> T>,
}

impl<T> StateBase<T> {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            _maker: PhantomData,
        }
    }
}
