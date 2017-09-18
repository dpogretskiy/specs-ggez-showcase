pub use physics::components::*;
pub use rendering::animation_seq::*;
pub use player::components::*;
use specs::*;

#[derive(Debug, Component, Copy, Clone)]
#[component(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position { x, y }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum RenderableType {
    Animation {
        id: &'static str,
        frame: usize,
        length: usize,
    },
    Image { id: &'static str },
    Batch { id: &'static str },
}

#[derive(Debug, Component)]
#[component(VecStorage)]
pub struct Renderable {
    pub layer: usize,
    pub tpe: RenderableType,
}

#[derive(Debug, Component, Default)]
#[component(NullStorage)]
pub struct SnapCamera;

#[derive(Debug, Component, Clone, Copy)]
#[component(HashMapStorage)]
pub enum Directional {
    Left,
    Right,
}

#[derive(Debug, Component, Clone, Copy)]
#[component(HashMapStorage)]
pub struct Scalable {
    pub x: f32,
    pub y: f32,
}

impl Scalable {
    pub fn new(x: f32, y: f32) -> Scalable {
        Scalable { x, y }
    }
}
