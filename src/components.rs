use specs::*;

#[derive(Debug, Component)]
#[component(HashMapStorage)]
pub struct Player;

#[derive(Debug, Component)]
#[component(VecStorage)]
pub struct Position{ pub x: f32, pub y: f32 }

#[derive(Debug, Component)]
#[component(VecStorage)]
pub enum Renderable { 
    Animation { id: &'static str, frame: usize },
    Image { id: &'static str },
    Batch { id: &'static str },
}
