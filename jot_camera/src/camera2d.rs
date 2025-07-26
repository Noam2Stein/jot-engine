use crevice::{std140::AsStd140, std430::AsStd430};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, AsStd140, AsStd430)]
pub struct Pos2Camera {
    pub center: SVec2,
    pub ortho_size: f32,
}

impl CameraType for Pos2Camera {
    type Inner = Self;

    fn update(&mut self, _timestep: s32) {}

    fn inner(&self) -> Self::Inner {
        *self
    }
}
