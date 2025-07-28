use super::*;

pub trait TileTransform2D: Transform2D {
    fn tile_pos(&self) -> IVec2;
}

impl TileTransform2D for Pos2D {
    fn tile_pos(&self) -> IVec2 {
        self.pos.floor().map(s32::as_i32).to_storage()
    }
}
