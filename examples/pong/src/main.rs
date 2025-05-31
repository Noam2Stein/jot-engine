use jot::{game::*, graphics::*, math::*, window::*};

fn main() {
    run::<Pong>();
}

struct Pong {}

impl Game for Pong {
    fn window_attrs() -> WindowAttributes {
        WindowAttributes::default().with_title("Pong")
    }

    fn new(_ctx: &GPUContext) -> Self {
        Self {}
    }

    fn update(&mut self, _delta_time: f64, _ctx: &GPUContext) -> GameFlow {
        GameFlow::Continue
    }

    fn draw(&mut self, output: &TextureView, ctx: &GPUContext) {
        clear(vec4(1.0, 0.0, 0.0, 0.0), output, ctx);
    }
}
