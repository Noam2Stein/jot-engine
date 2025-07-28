use std::{fs::File, marker::PhantomData, time::SystemTime};

use jot_gpu::Gpu;

use super::*;

#[derive(Debug)]
pub struct AssetId<T: AssetType> {
    pub(super) path: &'static str,
    t: PhantomData<T>,
}

impl<T: AssetType> AssetId<T> {
    pub fn load(self, gpu: &Gpu) -> Asset<T> {
        Asset::load(self, gpu)
    }

    pub const unsafe fn new_unchecked(path: &'static str) -> Self {
        Self {
            path,
            t: PhantomData,
        }
    }

    #[cfg(debug_assertions)]
    pub(super) fn has_changed_since(&self, time: SystemTime) -> bool {
        File::open(self.path).map_or(false, |file| {
            file.metadata().map_or(false, |metadata| {
                metadata
                    .modified()
                    .map_or(false, |modified| modified > time)
            })
        })
    }
}

impl<T: AssetType> Clone for AssetId<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: AssetType> Copy for AssetId<T> {}
