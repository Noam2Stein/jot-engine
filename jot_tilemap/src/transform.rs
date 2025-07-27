use std::cmp::Ordering;

use super::*;

pub trait TileTransform2D: Transform2D {
    fn tile_pos(&self) -> IVec2;

    fn tile_sort(&self, other: &Self) -> Ordering {
        let a = self.tile_pos();
        let b = other.tile_pos();

        if a.y() == b.y() {
            a.x().cmp(&b.x())
        } else {
            a.y().cmp(&b.y())
        }
    }

    fn tile_chunk<const CHUNK_HEIGHT: u32>(&self) -> i32 {
        (s32::from_i32(self.tile_pos().y()) / s32::from_u32(CHUNK_HEIGHT)).as_i32()
    }
}
