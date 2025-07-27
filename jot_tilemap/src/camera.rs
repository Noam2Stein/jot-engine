use std::ops::Range;

use super::*;

pub trait TilemapCamera2D: Camera2D {
    fn visible_tile_columns(&self) -> Range<i32>;

    fn visible_tile_chunks<const CHUNK_HEIGHT: u32>(&self) -> Range<i32>;
}
