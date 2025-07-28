use std::cmp::Ordering;

use super::*;

pub trait TileTransform2D: Transform2D {
    fn tile_pos(&self) -> IVec2;

    fn tile_chunk<const CHUNK_HEIGHT: u32>(&self) -> i32 {
        (s32::from_i32(self.tile_pos().y()) / s32::from_u32(CHUNK_HEIGHT)).as_i32()
    }
}

impl TileTransform2D for Pos2D {
    fn tile_pos(&self) -> IVec2 {
        self.pos.floor().map(s32::as_i32).to_storage()
    }
}
