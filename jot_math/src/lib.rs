pub use glam::{
    Affine2 as FAffine2, Affine3A as FAffine3A, Mat2 as FMat2, Mat3 as FMat3, Mat3A as FMat3A,
    Mat4 as FMat4, Quat as FQuat, Vec2 as FVec2, Vec3 as FVec3, Vec3A as FVec3A, Vec4 as FVec4,
    f64::*, i8::*, i16::*, i32::*, i64::*, mat2 as fmat2, mat3 as fmat3, mat3a as fmat3a,
    mat4 as fmat4, quat as fquat, u8::*, u16::*, u32::*, u64::*, usize::*, vec2 as fvec2,
    vec3 as fvec3, vec3a as fvec3a, vec4 as fvec4,
};

mod constants;
pub use constants::*;

pub mod bounds;
pub mod directions;
pub use bounds::*;
pub use directions::*;

pub mod s32_;
pub mod srect;
pub mod svec2;
pub mod svec3;
pub mod svec3a;
pub mod svec4;
pub use s32_::*;
pub use srect::*;
pub use svec2::*;
pub use svec3::*;
pub use svec3a::*;
pub use svec4::*;
