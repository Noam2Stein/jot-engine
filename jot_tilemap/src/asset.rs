use std::io::{Read, Write};

use jot_asset::*;

use proc_macro2::TokenStream;
use quote::quote;

use super::*;

impl<const CHUNK_HEIGHT: u32> AssetType for Tilemap<CHUNK_HEIGHT, Sprite, Pos2D> {
    type Metadata = ();

    fn type_path() -> TokenStream {
        quote! { jot::tilemap::Tilemap<#CHUNK_HEIGHT, jot::renderer2d::Sprite, jot::renderer2d::Pos2D> }
    }

    fn import(path: &std::path::Path, _meta: Option<Self::Metadata>, gpu: &Gpu) -> Self {
        if path.extension() != Some(std::ffi::OsStr::new("aseprite")) {
            panic!("Tilemap must currently be an aseprite file");
        }

        let file = asefile::AsepriteFile::read_file(path).unwrap();
        let tilemap = file.tilemap(0, 0).expect("expected tilemap");

        let tiles = (0..tilemap.width())
            .map(|x| (0..tilemap.height()).map(move |y| (x, y)))
            .flatten()
            .map(|(x, y)| (x, y, tilemap.tile(x, y).clone()))
            .map(|(x, y, tile)| Quad2D {
                visual: Sprite {
                    texture_rect: RectP::from_min_size(
                        vec2!(tile.id(), 0),
                        vec2!(tilemap.tile_size().0, tilemap.tile_size().1),
                    ),
                },
                transform: Pos2D {
                    pos: vec2p!(x, y).map(s32::from_u32),
                },
                depth: 0.0,
            })
            .collect::<Vec<_>>();

        Self::new(&tiles, gpu)
    }

    fn build(path: &std::path::Path, _meta: Option<Self::Metadata>, dst_path: &std::path::Path) {
        if path.extension() != Some(std::ffi::OsStr::new("aseprite")) {
            panic!("Tilemap must currently be an aseprite file");
        }

        let file = asefile::AsepriteFile::read_file(path).unwrap();
        let tilemap = file.tilemap(0, 0).expect("expected tilemap");

        let tiles = (0..tilemap.width())
            .map(|x| (0..tilemap.height()).map(move |y| (x, y)))
            .flatten()
            .map(|(x, y)| (x, y, tilemap.tile(x, y).clone()))
            .map(|(x, y, tile)| Quad2D {
                visual: Sprite {
                    texture_rect: RectP::from_min_size(
                        vec2!(tile.id(), 0),
                        vec2!(tilemap.tile_size().0, tilemap.tile_size().1),
                    ),
                },
                transform: Pos2D {
                    pos: vec2p!(x, y).map(s32::from_u32),
                },
                depth: 0.0,
            })
            .collect::<Vec<_>>();

        let structured = StructuredTiles::<CHUNK_HEIGHT, _, _>::new(&tiles);
        let serialized =
            bitcode::serialize(&structured).expect("failed to serialize StructuredTiles");

        let mut file = std::fs::File::create(dst_path).expect("failed to create file");
        file.write_all(&serialized).expect("failed to write file");
    }

    fn load(path: &std::path::Path, _gpu: &Gpu) -> Self {
        let mut file = std::fs::File::open(path).expect("failed to open file");

        let mut serialized = Vec::new();
        file.read_to_end(&mut serialized)
            .expect("failed to read file");

        let structured = bitcode::deserialize::<StructuredTiles<CHUNK_HEIGHT, _, _>>(&serialized)
            .expect("failed to deserialize StructuredTiles");

        Self::new_structured(&structured, _gpu)
    }
}
