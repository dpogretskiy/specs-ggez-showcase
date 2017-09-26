pub use physics::components::*;
pub use player::components::*;
pub use player::components::*;
pub use rendering::animation_seq::*;
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

impl RenderableType {
    pub fn set_animation_id(&mut self, new_id: &'static str, new_length: usize) {
        match self {
            &mut RenderableType::Animation {
                ref mut id,
                ref mut length,
                ..
            } => {
                *id = new_id;
                *length = new_length;
            }
            _ => (),
        }
    }
}

#[derive(Debug, Component)]
#[component(VecStorage)]
pub struct Renderable {
    pub layer: usize,
    pub tpe: RenderableType,
}

#[derive(Debug, Component)]
#[component(DenseVecStorage)]
pub struct Controlled;

#[derive(Debug, Component, Default)]
#[component(NullStorage)]
pub struct SnapCamera;

#[derive(Debug, Component, Default)]
#[component(NullStorage)]
pub struct ChaseCamera;

#[derive(Debug, Component, Clone, Copy)]
#[component(VecStorage)]
pub enum Directional {
    Left,
    Right,
}

#[derive(Debug, Component, Clone, Copy)]
#[component(DenseVecStorage)]
pub struct Scalable {
    pub x: f32,
    pub y: f32,
}

impl Scalable {
    pub fn new(x: f32, y: f32) -> Scalable {
        Scalable { x, y }
    }
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<MovingObject>();
    world.register::<HasAABB>();
    world.register::<Renderable>();
    world.register::<Scalable>();
    world.register::<Directional>();
    world.register::<HasAnimationSequence>();
    world.register::<PlayerStateMachine>();
    world.register::<Controlled>();
    world.register::<SnapCamera>();
    world.register::<StartPSM>();
    world.register::<ChaseCamera>();
    world.register::<CollisionDetection>();
}
