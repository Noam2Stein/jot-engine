use std::sync::Arc;
#[cfg(debug_assertions)]
use std::{sync::RwLock, time::SystemTime};

use jot_gpu::Gpu;

use super::*;

pub struct Asset<T: AssetType> {
    inner: T,

    #[cfg(debug_assertions)]
    generation: u32,
    #[cfg(debug_assertions)]
    ctx_generation: Arc<RwLock<u32>>,
    #[cfg(debug_assertions)]
    import_time: SystemTime,
    #[cfg(debug_assertions)]
    id: AssetId<T>,
}

impl<T: AssetType> Asset<T> {
    pub fn load(id: AssetId<T>, ctx: &AssetContext, gpu: &Gpu) -> Self {
        #[cfg(debug_assertions)]
        {
            use std::path::Path;

            let import_time = SystemTime::now();

            Self {
                inner: T::import(Path::new(id.path), meta, gpu),
                generation: *ctx.generation.read().unwrap(),
                ctx_generation: ctx.generation.clone(),
                import_time,
                id,
            }
        }

        #[cfg(not(debug_assertions))]
        {
            Self {
                inner: T::load(Path::new(id.path), gpu),
            }
        }
    }

    pub fn load_unchanging(id: AssetId<T>, gpu: &Gpu) -> Self {
        #[cfg(debug_assertions)]
        {
            use std::path::Path;

            let import_time = SystemTime::now();

            Self {
                inner: T::import(Path::new(id.path), meta, gpu),
                generation: 0,
                ctx_generation: Arc::new(RwLock::new(0)),
                import_time,
                id,
            }
        }

        #[cfg(not(debug_assertions))]
        {
            Self {
                inner: T::load(Path::new(id.path), gpu),
            }
        }
    }

    pub fn get(&self) -> &T {
        &self.inner
    }

    /// Refreshes the asset only if its `AssetContext` has been refreshed.
    /// This is meant to be called alot, and does nothing on release mode.
    pub fn refresh(&mut self, gpu: &Gpu) {
        #[cfg(debug_assertions)]
        {
            let ctx_generation = *self.ctx_generation.read().unwrap();

            if self.generation < ctx_generation {
                self.generation = ctx_generation;

                if self.id.has_changed_since(self.import_time) {
                    *self = Self::load(
                        self.id,
                        &AssetContext {
                            generation: self.ctx_generation.clone(),
                        },
                        gpu,
                    );
                }
            }
        }
    }
}
