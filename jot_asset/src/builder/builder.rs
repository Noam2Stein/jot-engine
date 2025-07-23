use std::{
    collections::HashMap,
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use proc_macro2::TokenStream;
use quote::ToTokens;
use serde::Deserialize;

use super::*;

/// Asset building tool made for `build.rs` files.
#[derive(Default)]
pub struct AssetBuilder {
    ext_defaults: HashMap<String, String>,
    types: HashMap<String, AssetTypeInfo>,
}

impl AssetBuilder {
    /// Expects extensions without the dot.
    pub fn set_ext_default(&mut self, ext: impl Into<String>, asset_kind: impl Into<String>) {
        self.ext_defaults.insert(ext.into(), asset_kind.into());
    }

    pub fn set_type<T: AssetType>(&mut self, asset_kind: impl Into<String>) {
        self.types.insert(
            asset_kind.into(),
            AssetTypeInfo {
                type_path: T::type_path(),
                _build_fn: |path, meta, dst_path| {
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
        let assets_dir = manifest_dir.join("assets");
        let src_assets_file = manifest_dir.join("src/assets.rs");

        let mut assets_mod = AssetsMod::new(assets_dir.clone());

        for asset_path in find_assets(&assets_dir) {
            let meta = read_metadata::<AssetMetadata>(&asset_path);

            let asset_kind = self.get_asset_kind(&asset_path, meta.as_ref());

            let asset_type = self.types.get(&asset_kind).expect(&format!(
                "failed to select asset data-type for `{asset_kind}` kind"
            ));

            assets_mod.push(
                asset_path.strip_prefix(&assets_dir).unwrap(),
                &asset_type.type_path,
            );
        }

        File::create(&src_assets_file)
            .expect("failed to create `assets.rs`")
            .write_fmt(format_args!("{}", assets_mod.to_token_stream().to_string()))
            .expect("failed to write to `assets.rs`");
    }

    fn get_asset_kind(&self, asset_path: &Path, meta: Option<&AssetMetadata>) -> String {
        if let Some(meta) = meta.as_ref() {
            meta.asset_kind.clone()
        } else {
            self.ext_defaults
                .get(
                    &asset_path
                        .extension()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                )
                .expect(&format!(
                    "failed to select asset kind for \"{}\"",
                    asset_path.display()
                ))
                .clone()
        }
    }
}

#[derive(Deserialize)]
struct AssetMetadata {
    asset_kind: String,
}

struct AssetTypeInfo {
    type_path: TokenStream,
    _build_fn: fn(&Path, Option<&str>, &Path),
}
