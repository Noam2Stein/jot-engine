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

        let assets = find_assets(&assets_dir);

        let assets_mod_str = match assets {
            Ok(assets) => {
                let mut assets_mod = AssetsMod::new(assets_dir.clone());

                for asset_path in assets {
                    let type_path = self.build_asset(&asset_path);

                    assets_mod.push(asset_path.strip_prefix(&assets_dir).unwrap(), type_path);
                }

                assets_mod.to_string()
            }

            Err(err) => err.to_token_stream().to_string(),
        };

        File::create(&src_assets_file)
            .expect("failed to create `assets.rs`")
            .write(assets_mod_str.as_bytes())
            .expect("failed to write to `assets.rs`");
    }

    fn build_asset(&self, asset_path: &Path) -> BuilderResult<TokenStream> {
        let meta = read_metadata::<AssetMetadata>(&asset_path);

        let asset_kind = self.get_asset_kind(&asset_path, meta.as_ref())?;

        let asset_type = self.get_asset_type(&asset_kind)?;

        Ok(asset_type.type_path.clone())
    }

    fn get_asset_kind(
        &self,
        asset_path: &Path,
        meta: Option<&AssetMetadata>,
    ) -> BuilderResult<String> {
        if let Some(meta) = meta.as_ref() {
            Ok(meta.asset_kind.clone())
        } else if let Some(default) = self.ext_defaults.get(
            &asset_path
                .extension()
                .unwrap()
                .to_string_lossy()
                .to_string(),
        ) {
            Ok(default.clone())
        } else {
            Err(builder_error!(
                "failed to select asset kind for \"{}\"",
                asset_path.display()
            ))
        }
    }

    fn get_asset_type(&self, asset_kind: &String) -> BuilderResult<&AssetTypeInfo> {
        match self.types.get(asset_kind) {
            Some(asset_type) => Ok(asset_type),

            None => Err(builder_error!(
                "failed to select asset data-type for `{asset_kind}` kind"
            )),
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
