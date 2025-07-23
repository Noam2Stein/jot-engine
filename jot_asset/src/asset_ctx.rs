#[cfg(debug_assertions)]
use std::sync::{Arc, RwLock};

#[derive(Debug, Default)]
pub struct AssetContext {
    #[cfg(debug_assertions)]
    pub(super) generation: Arc<RwLock<u32>>,
}

impl AssetContext {
    pub fn new() -> Self {
        Self {
            #[cfg(debug_assertions)]
            generation: Arc::new(RwLock::new(0)),
        }
    }

    pub fn refresh(&self) {
        #[cfg(debug_assertions)]
        {
            *self.generation.write().unwrap() += 1;
        }
    }
}
