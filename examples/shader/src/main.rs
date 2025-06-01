use jot::{
    game::{Game, run},
    shader::shader_mod,
};

fn main() {
    run::<ShaderGame>();
}

struct ShaderGame {}

impl Game for ShaderGame {
    fn new(_ctx: &jot::graphics::GPUContext) -> Self {
        Self {}
    }
}

shader_mod! {
    example_shader:

    struct Vertex {

    }

    pub struct G {

    }

    pub fn g() {}
}
