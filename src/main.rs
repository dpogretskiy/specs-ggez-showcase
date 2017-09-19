extern crate ggez;
extern crate marker;
extern crate nalgebra as na;
extern crate rand;
extern crate rayon;
extern crate serde_json;
extern crate specs;
#[macro_use]
extern crate specs_derive;

extern crate cpuprofiler;

mod sprite;
mod components;
mod systems;
mod resources;
mod physics;
mod util;
mod level;
mod game;
mod rendering;
mod player;

pub use rendering::asset_storage;
pub use rendering::camera;

use game::*;
use ggez::{Context, conf, event, graphics};

// use cpuprofiler::PROFILER;

fn main() {
    let c = conf::Conf {
        window_width: 1600,
        window_height: 1000,
        resizable: false,
        vsync: false,
        ..Default::default()
    };
    let ctx = &mut Context::load_from_conf("config", "me", c).unwrap();
    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

    let mut state = Game::new(ctx).unwrap();

    event::run(ctx, &mut state).unwrap();
}
