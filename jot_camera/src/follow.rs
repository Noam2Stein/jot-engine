use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirectFollow<C: CameraType> {
    cam: C,
}

impl DirectFollow<Pos2Camera> {
    pub fn new(target: SVec2, ortho_size: f32) -> Self {
        Self {
            cam: Pos2Camera {
                center: target,
                ortho_size,
            },
        }
    }

    pub fn target_moved(&mut self, target: SVec2) {
        self.cam.center = target;
    }
}
impl CameraType for DirectFollow<Pos2Camera> {
    type Inner = Pos2Camera;

    fn update(&mut self, _timestep: s32) {}

    fn inner(&self) -> Self::Inner {
        self.cam
    }
}
