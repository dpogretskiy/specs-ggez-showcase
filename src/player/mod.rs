pub mod components;
pub mod systems;
pub mod animation_defs;
pub mod state_machine;
pub mod state;
mod consts;
mod animation_loader;

use self::animation_defs::*;
pub use self::animation_loader::AnimationLoader;
pub use self::consts::*;

use components::*;
use physics::AABB;
use specs::World;
use util::Vector2;

pub struct Player;
impl Player {
    pub fn spawn(
        world: &mut World,
        location: Vector2,
        controlled: bool,
        camera_snap: bool,
        count: &mut usize,
    ) {

        let psm = PlayerStateMachine { machine: state_machine::StateMachine::new(state::Idle) };

        let pos = Position::new(location.x as f32, location.y as f32);
        let player_scale: f64 = 0.4;
        let scalable = Scalable::new(player_scale as f32, player_scale as f32);

        let e = world
            .create_entity()
            .with(pos)
            .with(Renderable {
                layer: 5,
                tpe: RenderableType::Animation {
                    id: "player-idle",
                    frame: 0,
                    length: 10,
                },
            })
            .with(HasAnimationSequence { sequence: PlayerAnimations::idle() })
            .with(psm)
            .with(StartPSM)
            .with(Directional::Right)
            .with(scalable)
            .with(MovingObject::new(location.clone()))
            .with(HasAABB::new(AABB::new_full(
                Vector2::new(290.0, 500.0) * player_scale,
                Vector2::new(0.7, 0.8),
            )))
            .with(CollisionDetection { group: 0 });

        let e = if camera_snap { e.with(SnapCamera) } else { e };

        let e = if controlled { e.with(Controlled) } else { e };

        e.build();

        *count += 1;
        println!("Players: {}", count);
    }
}
