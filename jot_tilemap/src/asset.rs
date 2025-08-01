use jot_asset::*;

use proc_macro2::TokenStream;
use quote::quote;

use super::*;

impl<const CHUNK_HEIGHT: usize> AssetType for Tilemap<CHUNK_HEIGHT, Sprite, Pos2D> {
    fn type_path() -> TokenStream {
        quote! { jot::tilemap::Tilemap<#CHUNK_HEIGHT, jot::renderer2d::Sprite, jot::renderer2d::Pos2D> }
    }

    fn import(path: &std::path::Path, meta: Option<Self::Metadata>, _gpu: &Gpu) -> Self {
        if path.extension() != Some(std::ffi::OsStr::new("aseprite")) {
            panic!("Tilemap must currently be an aseprite file");
        }

        let file = asefile::AsepriteFile::read_file(path).unwrap();
        let tilemap = file.tilemap(0, 0).expect("expected tilemap");
    }
}
