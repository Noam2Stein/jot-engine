use std::ops::{Deref, DerefMut};

use noise::{NoiseFn, Perlin};

use super::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Shake<T: CameraType> {
    base: T,
    shaked: T::Inner,
    perlin_x: Perlin,
    perlin_y: Perlin,
    time: s32,

    radius: s32,
    slowdown: s32,
    speed: s32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShakeDesc {
    pub radius: s32,
    pub slowdown: s32,
    pub speed: s32,
}

impl<C: CameraType> Shake<C> {
    pub fn new(base: C) -> Self {
        Self {
            shaked: base.inner(),
            base,
            perlin_x: Perlin::new(5),
            perlin_y: Perlin::new(2401),
            time: s32::ZERO,
            radius: s32::ZERO,
            slowdown: s32::ZERO,
            speed: s32::ZERO,
        }
    }

    pub fn shake(&mut self, desc: ShakeDesc) {
        if desc.radius > self.radius {
            self.radius = desc.radius;
            self.slowdown = desc.slowdown;
            self.speed = desc.speed;
        }
    }
}
impl<C: CameraType<Inner = Pos2Camera>> CameraType for Shake<C> {
    type Inner = Pos2Camera;

    fn update(&mut self, timestep: s32) {
        self.base.update(timestep);

        let perlin_point = [self.time.as_f64()];
        let perlin = vec2p!(
            self.perlin_x.get(perlin_point),
            self.perlin_y.get(perlin_point)
        );

        self.shaked = Pos2Camera {
            center: self.base.inner().center + perlin.map(s32::from_f64) * self.radius,
            ortho_size: self.base.inner().ortho_size,
        };

        self.radius = self
            .radius
            .moved_towards(s32::ZERO, timestep * self.slowdown);

        if self.radius == s32::ZERO {
            // speed doesn't matter while theres no shake, and this delays time overflow.
            self.speed = s32::ZERO;
        }

        self.time += timestep * self.speed;
    }

    fn inner(&self) -> Self::Inner {
        self.shaked
    }
}

impl<C: CameraType> Deref for Shake<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<C: CameraType> DerefMut for Shake<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ShakeDesc {
    pub const LIGHT: Self = Self {
        radius: s32::from_f32(0.15),
        slowdown: s32::from_f32(1.0),
        speed: s32::from_f32(10.0),
    };

    pub const HEAVY: Self = Self {
        radius: s32::from_f32(0.25),
        slowdown: s32::from_f32(1.0),
        speed: s32::from_f32(15.0),
    };
}
