use jot::{
    camera::*, chunk::*, collections::*, fixed::*, game::*, gpu::*, input::*, math::*,
    renderer2d::*, scene::*, scheme::*,
};

mod game;
mod gameplay;
mod input;
mod renderer;

fn main() {
    run::<game::Game>();
}
