use std::ops::Range;

use super::*;

pub trait TilemapCamera2D: Camera2D {
    fn visible_tile_columns(&self, aspect: f32) -> Range<i32>;

    fn visible_tile_chunks<const CHUNK_HEIGHT: u32>(&self, aspect: f32) -> Range<i32>;
}

impl TilemapCamera2D for Pos2Camera {
    fn visible_tile_columns(&self, aspect: f32) -> Range<i32> {
        let left = self.center.x() - s32::from_f32(self.ortho_size * aspect);
        let right = self.center.x() + s32::from_f32(self.ortho_size * aspect);

        let start = left.floor().as_i32();
        let end_inclusive = right.ceil().as_i32();

        let end = end_inclusive + 1;

        start..end
    }

    fn visible_tile_chunks<const CHUNK_HEIGHT: u32>(&self, _aspect: f32) -> Range<i32> {
        let bottom = self.center.y() - s32::from_f32(self.ortho_size);
        let top = self.center.y() + s32::from_f32(self.ortho_size);

        let start = (bottom / s32::from_u32(CHUNK_HEIGHT)).floor().as_i32();
        let end_inclusive = (top / s32::from_u32(CHUNK_HEIGHT)).ceil().as_i32();

        let end = end_inclusive + 1;

        start..end
    }
}
