use std::{marker::PhantomData, mem::replace};

/// Vector that gives each item a handle, while keeping items continious in memory.
#[derive(Debug, Default)]
pub struct HandleVec<T> {
    buf: Vec<T>,
    indicies: Vec<Option<usize>>,
}

/// A handle to an item in a `HandleVec`.
///
/// Does not know what `HandleVec` it came from,
/// so when using this ensure that handles are only used with their original vec.
#[must_use]
#[derive(Debug, Hash)]
pub struct Handle<T> {
    idx_idx: usize,
    t: PhantomData<T>,
}

impl<T> HandleVec<T> {
    pub fn new() -> Self {
        Self {
            buf: Vec::new(),
            indicies: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
            indicies: Vec::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, value: T) -> Handle<T> {
        let idx = self.buf.len();
        self.buf.push(value);

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
        }
    }

    /// When calling this, ensure that `handle` originally comes from this vector.
    pub fn remove(&mut self, handle: Handle<T>) {
        let idx = replace(&mut self.indicies[handle.idx_idx], None).unwrap();

        self.buf.swap_remove(idx);
        let swapped_idx = self.buf.len();

        if let Some(swapped_idx_slot) = self
            .indicies
            .iter_mut()
            .find(|idx_slot| **idx_slot == Some(swapped_idx))
        {
            *swapped_idx_slot = Some(idx);
        }
    }

    /// When calling this, ensure that `handle` originally comes from this vector.
    pub fn get(&self, handle: &Handle<T>) -> &T {
        let idx = self.indicies[handle.idx_idx].unwrap();

        &self.buf[idx]
    }

    /// Doesn't require `&mut self` because mut safety is tracked by the handle, not vector.
    /// This allows mutating multiple vector items at the same time.
    ///
    /// When calling this, ensure that `handle` originally comes from this vector.
    pub fn get_mut(&self, handle: &mut Handle<T>) -> &mut T {
        let idx = self.indicies[handle.idx_idx].unwrap();

        unsafe { &mut *(self.buf.as_ptr().add(idx) as *const _ as *mut _) }
    }

    /// Requires `&mut self` because items can be mutated even with `&self` if they have a mut handle reference.
    pub fn as_slice(&mut self) -> &[T] {
        &self.buf
    }
}
