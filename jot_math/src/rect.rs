use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub center: Vec2,
    pub extents: Vec2,
}

impl Rect {
    pub const fn new(center: Vec2, extents: Vec2) -> Self {
        Self { center, extents }
    }
}
