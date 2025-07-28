use std::{fmt::Debug, hash::Hash};

use super::*;

pub use jot_scheme_proc_macros::InputType;

pub trait InputType: Debug + Copy + Eq + Hash + Default {
    type Bindings: Debug + Clone + Eq + Default;
    type ResolverState: Debug + Clone + Default;

    fn new_resolver(bindings: Self::Bindings) -> Self::ResolverState;

    fn event(resolver: &mut Self::ResolverState, event: &InputDeviceEvent);

    fn step(resolver: &mut Self::ResolverState) -> Self;
}

pub type Bindings<T> = <T as InputType>::Bindings;

#[derive(Debug, Clone, Default)]
pub struct Resolver<T: InputType> {
    state: T::ResolverState,
}

impl<T: InputType> Resolver<T> {
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
