use jot::game::{Game, run};

fn main() {
    run::<EmptyGame>();
}

struct EmptyGame {}

impl Game for EmptyGame {
    const NAME: &str = "Empty Game";

    fn new(_gpu: &jot::gpu::Gpu) -> Self {
        Self {}
    }
}
