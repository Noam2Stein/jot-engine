use std::{fmt::Debug, hash::Hash};

use jot_input::*;

mod button;
mod flat_binding;
mod value_binding;
pub use button::*;
pub use flat_binding::*;
pub use value_binding::*;

pub trait InputState: Debug + Copy + Eq + Hash + Default {
    type Bindings: InputBindings;
    type ResolverState: Debug + Clone + Default;

    fn new_resolver(bindings: Self::Bindings) -> Self::ResolverState;

    fn event(resolver: &mut Self::ResolverState, event: &InputDeviceEvent);

    fn step(resolver: &mut Self::ResolverState) -> Self;
}

pub trait InputBindings: Default {}

#[derive(Debug, Clone, Default)]
pub struct InputResolver<T: InputState> {
    state: T::ResolverState,
}

impl<T: InputState> InputResolver<T> {
    pub fn new(bindings: T::Bindings) -> Self {
        Self {
            state: T::new_resolver(bindings),
        }
    }

    pub fn event(&mut self, event: &InputDeviceEvent) {
        T::event(&mut self.state, event);
    }

    pub fn step(&mut self) -> T {
        T::step(&mut self.state)
    }
}
