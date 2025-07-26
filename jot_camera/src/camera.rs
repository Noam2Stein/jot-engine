use crevice::{std140::AsStd140, std430::AsStd430};

use super::*;

/// Acts as a camera with its center at `(0, 0)` and its ortho_size as `1.0`.
///
/// In 2D this acts like passing raw `-1` to `1` coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, AsStd140, AsStd430)]
pub struct NoCamera {}

impl CameraType for NoCamera {
    type Inner = Self;

    fn update(&mut self, _timestep: s32) {}

    fn inner(&self) -> Self::Inner {
        *self
    }
}
