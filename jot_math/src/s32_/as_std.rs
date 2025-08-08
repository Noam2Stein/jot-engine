use crevice::{std140::AsStd140, std430::AsStd430};
use ggmath::crevice_::{ScalarAsStd140, ScalarAsStd430};

use super::*;

impl AsStd140 for s32 {
    type Output = <i32 as AsStd140>::Output;

    fn as_std140(&self) -> Self::Output {
        self.0.as_std140()
    }

    fn from_std140(val: Self::Output) -> Self {
        Self(i32::from_std140(val))
    }
}

impl AsStd430 for s32 {
    type Output = <i32 as AsStd430>::Output;

    fn as_std430(&self) -> Self::Output {
        self.0.as_std430()
    }

    fn from_std430(val: Self::Output) -> Self {
        Self(i32::from_std430(val))
    }
}

impl ScalarAsStd140 for s32 {
    type OutputVec2 = <i32 as ScalarAsStd140>::OutputVec2;
    type OutputVec3 = <i32 as ScalarAsStd140>::OutputVec3;
    type OutputVec4 = <i32 as ScalarAsStd140>::OutputVec4;

    fn std140_from_vec2(vec: Self::OutputVec2) -> [Self::Output; 2] {
        <i32 as ScalarAsStd140>::std140_from_vec2(vec)
    }
    fn std140_from_vec3(vec: Self::OutputVec3) -> [Self::Output; 3] {
        <i32 as ScalarAsStd140>::std140_from_vec3(vec)
    }
    fn std140_from_vec4(vec: Self::OutputVec4) -> [Self::Output; 4] {
        <i32 as ScalarAsStd140>::std140_from_vec4(vec)
    }

    fn std140_to_vec2(value: [Self::Output; 2]) -> Self::OutputVec2 {
        <i32 as ScalarAsStd140>::std140_to_vec2(value)
    }
    fn std140_to_vec3(value: [Self::Output; 3]) -> Self::OutputVec3 {
        <i32 as ScalarAsStd140>::std140_to_vec3(value)
    }
    fn std140_to_vec4(value: [Self::Output; 4]) -> Self::OutputVec4 {
        <i32 as ScalarAsStd140>::std140_to_vec4(value)
    }
}

impl ScalarAsStd430 for s32 {
    type OutputVec2 = <i32 as ScalarAsStd430>::OutputVec2;
    type OutputVec3 = <i32 as ScalarAsStd430>::OutputVec3;
    type OutputVec4 = <i32 as ScalarAsStd430>::OutputVec4;

    fn std430_from_vec2(vec: Self::OutputVec2) -> [Self::Output; 2] {
        <i32 as ScalarAsStd430>::std430_from_vec2(vec)
    }
    fn std430_from_vec3(vec: Self::OutputVec3) -> [Self::Output; 3] {
        <i32 as ScalarAsStd430>::std430_from_vec3(vec)
    }
    fn std430_from_vec4(vec: Self::OutputVec4) -> [Self::Output; 4] {
        <i32 as ScalarAsStd430>::std430_from_vec4(vec)
    }

    fn std430_to_vec2(value: [Self::Output; 2]) -> Self::OutputVec2 {
        <i32 as ScalarAsStd430>::std430_to_vec2(value)
    }
    fn std430_to_vec3(value: [Self::Output; 3]) -> Self::OutputVec3 {
        <i32 as ScalarAsStd430>::std430_to_vec3(value)
    }
    fn std430_to_vec4(value: [Self::Output; 4]) -> Self::OutputVec4 {
        <i32 as ScalarAsStd430>::std430_to_vec4(value)
    }
}
