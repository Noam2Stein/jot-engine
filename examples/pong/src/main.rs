use jot::{context::*, game::*, graphics::*, math::*, window::*};

fn main() {
    run::<Pong>();
}

struct Pong {}

impl Game for Pong {
    fn window_attrs() -> WindowAttributes {
        WindowAttributes::default().with_title("Pong")
    }

    fn new(_ctx: &Context) -> Self {
        Self {}
    }

    fn update(&mut self, _delta_time: f64, _ctx: &Context) -> GameFlow {
        GameFlow::Continue
    }

    fn draw(&mut self, output: &jot::graphics::TextureView, ctx: &Context) {
        clear(vec4(1.0, 0.0, 0.0, 0.0), output, ctx);
    }
}
