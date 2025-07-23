use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
};

use proc_macro2::TokenStream;

use super::*;

/// Asset building tool made for `build.rs` files.
#[derive(Default)]
pub struct AssetBuilder {
    ext_defaults: HashMap<String, String>,
    handlers: HashMap<String, AssetTypeInfo>,
}

impl AssetBuilder {
    /// Expects extensions without the dot.
    pub fn set_ext_default(&mut self, ext: impl Into<String>, asset_kind: impl Into<String>) {
        self.ext_defaults.insert(ext.into(), asset_kind.into());
    }

    pub fn set_type<T: AssetType>(&mut self, asset_kind: impl Into<String>) {
        self.handlers.insert(
            asset_kind.into(),
            AssetTypeInfo {
                type_path: T::type_path(),
                build_fn: |path, meta, dst_path| {
                    let meta = meta.map(|meta| {
                        serde_yaml::from_str(meta).expect("failed to deserialize asset metadata")
                    });

                    T::build(path, meta, dst_path);
                },
            },
        );
    }

    /// Only works in a `build.rs` context.
    pub fn build(self) {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let src_assets_dir = manifest_dir.join("src/assets");
        let assets_dir = manifest_dir.join("assets");
    }
}

struct AssetTypeInfo {
    type_path: TokenStream,
    build_fn: fn(&Path, Option<&str>, &Path),
}
