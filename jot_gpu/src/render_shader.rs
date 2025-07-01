use std::fmt::Debug;

#[doc(hidden)]
pub use jot_gpu_proc_macros::render_shader as render_shader_proc;

use super::*;

#[macro_export]
macro_rules! render_shader {
    { $($input:tt)* } => {
        $crate::render_shader_proc! {
            $crate::self;
            $($input)*
        }
    };
}

pub trait RenderShader: Debug + Copy {
    type Vertex: GpuVertex;
    type Uniforms;

    const WGSL: &str;
}

pub trait GpuVertex: Debug + Copy + PartialEq {
    type Layout: GpuVertexLayout;
}
pub trait GpuVertexLayout {
    fn attributes(&self, attrs: &mut Vec<wgpu::VertexAttribute>);
}

pub trait GpuVertexBindings<V: GpuVertex> {}
