extern crate ggez;
extern crate marker;
extern crate nalgebra as na;
extern crate rand;
extern crate rayon;
extern crate serde_json;
extern crate specs;
#[macro_use]
extern crate specs_derive;

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
use ggez::{conf, event, graphics, Context};

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
    println!("{:?}", graphics::get_renderer_info(ctx).unwrap());

    let mut state = Game::new(ctx).unwrap();

    event::run(ctx, &mut state).unwrap();
}




//jerk off
type Lifetimed<'a> = &'a usize;

struct LTStruct<T> {
    no_lifetime: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T> LTStruct<T> {
    pub fn new(x: usize) -> LTStruct<T> {
        use std::marker::PhantomData;

        LTStruct {
            no_lifetime: x,
            _marker: PhantomData,
        }
    }

    pub fn xxxx(&self, x: T) -> T {
        x
    }
}

struct So {
    pub s: LTStruct<Lifetimed<_>>,
}

fn nailed_it<'a>(s: So) -> Lifetimed<'a> {
    let us = 15;
    s.s.xxxx(&us)
}
