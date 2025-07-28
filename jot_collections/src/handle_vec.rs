use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::replace,
    sync::{Arc, Mutex, Weak},
};

use super::*;

/// Vector that gives each item a handle, while keeping items continious in memory.
#[derive(Debug, Default, Component, Resource)]
pub struct HandleVec<T> {
    buf: UnsafeCell<Vec<T>>,
    indicies: Vec<Option<usize>>,
    remove: Arc<Mutex<Vec<usize>>>,
}

/// A handle to an item in a `HandleVec`.
///
/// Does not know what `HandleVec` it came from,
/// so when using this ensure that handles are only used with their original vector.
///
/// When a handle is dropped, it is automatically removed from the vector.
#[must_use]
#[derive(Debug, Component)]
pub struct Handle<T> {
    idx_idx: usize,
    t: PhantomData<T>,
    remove: Weak<Mutex<Vec<usize>>>,
}

impl<T> HandleVec<T> {
    pub fn new() -> Self {
        Self {
            buf: UnsafeCell::new(Vec::new()),
            indicies: Vec::new(),
            remove: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: UnsafeCell::new(Vec::with_capacity(capacity)),
            indicies: Vec::with_capacity(capacity),
            remove: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn insert(&mut self, value: T) -> Handle<T> {
        self.remove_dropped();

        let idx = self.buf.get_mut().len();
        self.buf.get_mut().push(value);

        let idx_idx = if let Some((idx_idx, idx_slot)) = self
            .indicies
            .iter_mut()
            .enumerate()
            .find(|(_, idx_slot)| idx_slot.is_none())
        {
            *idx_slot = Some(idx);
            idx_idx
        } else {
            let idx_idx = self.indicies.len();
            self.indicies.push(Some(idx));

            idx_idx
        };

        Handle {
            idx_idx,
            t: PhantomData,
            remove: Arc::downgrade(&self.remove),
        }
    }

    /// When calling this, ensure that `handle` originally comes from this vector.
    pub fn get(&self, handle: &Handle<T>) -> &T {
        let idx = self.indicies[handle.idx_idx].unwrap();

        unsafe { &*(&*self.buf.get()).as_ptr().add(idx) }
    }

    /// Doesn't require `&mut self` because mut safety is tracked by the handle, not vector.
    /// This allows mutating multiple vector items at the same time.
    ///
    /// When calling this, ensure that `handle` originally comes from this vector.
    pub fn get_mut(&self, handle: &mut Handle<T>) -> &mut T {
        let idx = self.indicies[handle.idx_idx].unwrap();

        unsafe { &mut *(&mut *self.buf.get()).as_mut_ptr().add(idx) }
    }

    /// Requires `&mut self` because items can be mutated even with `&self` if they have a mut handle reference.
    pub fn as_slice(&mut self) -> &[T] {
        self.remove_dropped();

        self.buf.get_mut()
    }

    fn remove_dropped(&mut self) {
        for idx_idx in self.remove.lock().unwrap().drain(..) {
            let idx = replace(&mut self.indicies[idx_idx], None).unwrap();

            self.buf.get_mut().swap_remove(idx);
            let swapped_idx = self.buf.get_mut().len();

            if let Some(swapped_idx_slot) = self
                .indicies
                .iter_mut()
                .find(|idx_slot| **idx_slot == Some(swapped_idx))
            {
                *swapped_idx_slot = Some(idx);
            }
        }
    }
}

unsafe impl<T: Send> Send for HandleVec<T> {}
unsafe impl<T: Sync> Sync for HandleVec<T> {}

impl<T> Drop for Handle<T> {
    fn drop(&mut self) {
        if let Some(remove) = self.remove.upgrade() {
            remove.lock().unwrap().push(self.idx_idx);
        }
    }
}
