use jot::game::{Game, run};

fn main() {
    run::<EmptyGame>();
}

struct EmptyGame {}

impl Game for EmptyGame {
    fn new(_ctx: &jot::graphics::GPUContext) -> Self {
        Self {}
    }
}
