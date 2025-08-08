use jot::{
    camera::*, collections::*, fixed::*, game::*, gpu::*, input::*, math::*, renderer2d::*,
    scene::*, scheme::*, tilemap::*,
};

mod game;
mod gameplay;
mod input;

fn main() {
    run::<game::Game>();
}
