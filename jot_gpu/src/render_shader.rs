use std::fmt::Debug;

use super::*;

pub trait RenderShader {
    type Vertex: VertexType;
    type Uniforms;

    const WGSL: &str;
}

pub trait VertexType: Debug + Copy + PartialEq {
    type Layout: VertexLayoutType;
}
pub trait VertexLayoutType {
    fn 
}