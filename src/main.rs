extern crate ggez;
extern crate nalgebra as na;
extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate marker;
extern crate rand;

mod components;
mod systems;
mod resources;
mod physics;

type Vector2 = na::Vector2<f64>;

fn main() {
    println!("Ahoy matey");
}
