use std::path::Path;

use jot_gpu::Gpu;
use proc_macro2::TokenStream;
use serde::Deserialize;

pub trait AssetType {
    type Metadata: for<'a> Deserialize<'a>;

    fn type_path() -> TokenStream;

    fn import(path: &Path, meta: Option<Self::Metadata>, _gpu: &Gpu) -> Self;

    fn build(path: &Path, meta: Option<Self::Metadata>, dst_path: &Path);

    fn load(path: &Path, _gpu: &Gpu) -> Self;
}
