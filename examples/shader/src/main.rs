use jot::{game::*, shader::*};

fn main() {
    run::<ShaderGame>();
}

struct ShaderGame {}

impl Game for ShaderGame {
    fn new(_ctx: &jot::graphics::GPUContext) -> Self {
        Self {}
    }
}

shader_interface! {
    transform:

    type Transform;

    fn transform(vertex: FVec3) -> FVec3;
}

shader_mod! {
    example_shader<T: transform>:

    struct Vertex {
        vertex: FVec3,
        transform: T::Transform,
    }

    pub struct G {
        d: Vertex,
    }

    pub fn g() {}
}
