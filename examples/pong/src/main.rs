use jot::{game::*, gpu::*, math::*};

fn main() {
    run::<Pong>();
}

struct Pong {}

impl Game for Pong {
    const NAME: &str = "Pong";

    fn new(_gpu: &Gpu) -> Self {
        Self {}
    }

    fn update(&mut self, _delta_time: f64, _gpu: &Gpu) -> GameFlow {
        GameFlow::Continue
    }

    fn draw(&self, output: &GpuTexture2DView, gpu: &Gpu) {
        output.clear(fvec4(0.0, 1.0, 1.0, 0.0), gpu);
    }
}
