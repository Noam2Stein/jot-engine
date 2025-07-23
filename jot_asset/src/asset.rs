#[cfg(debug_assertions)]
use std::time::SystemTime;

use super::*;

pub struct Asset<T: AssetType> {
    inner: T,

    #[cfg(debug_assertions)]
    import_time: SystemTime,
    #[cfg(debug_assertions)]
    id: AssetId<T>,
}

impl<T: AssetType> Asset<T> {
    pub fn load(id: AssetId<T>, gpu: &Gpu) -> Self {
        #[cfg(debug_assertions)]
        {
            use std::path::Path;

            let inner = T::import(Path::new(id.path), read_metadata(Path::new(id.path)), gpu);

            let import_time = SystemTime::now();

            Self {
                inner,
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
}

impl<T: AssetType> HotReload for Asset<T> {
    fn hot_reload(&mut self, gpu: &Gpu) {
        #[cfg(debug_assertions)]
        if self.id.has_changed_since(self.import_time) {
            *self = Self::load(self.id, gpu);
        }
    }
}
