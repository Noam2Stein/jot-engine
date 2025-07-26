use std::fmt::Debug;

use super::*;

/// Any type that stores a camera.
///
/// This can either be directly a camera state type that can be passed to a renderer, or a type that moves the camera and can be updated.
pub trait CameraType: Debug {
    type Inner: Debug + Copy;

    fn update(&mut self, timestep: s32);

    fn inner(&self) -> Self::Inner;
}
