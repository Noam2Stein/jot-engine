use jot::game::{GameType, run};

fn main() {
    run::<EmptyGame>();
}

struct EmptyGame {}

impl GameType for EmptyGame {
    const NAME: &str = "Empty Game";

    fn new(_gpu: &jot::gpu::Gpu) -> Self {
        Self {}
    }
}
