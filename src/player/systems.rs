use components::*;
use player::components::*;
use player::consts as PC;
use resources::*;
use specs::*;

pub struct PlayerDirectionSystem;
impl<'a> System<'a> for PlayerDirectionSystem {
    type SystemData = (
        WriteStorage<'a, Directional>,
        ReadStorage<'a, Controlled>,
        Fetch<'a, PlayerInput>,
    );

    fn run(&mut self, (mut directional, controlled, input): Self::SystemData) {
        for (mut dir, _) in (&mut directional, &controlled).join() {
            if input.left ^ input.right {
                if input.left {
                    *dir = Directional::Left;
                } else {
                    *dir = Directional::Right;
                }
            }
        }
    }
}

pub type PlayerSystemData<'a> = (
    Entities<'a>,
    ReadStorage<'a, Controlled>,
    WriteStorage<'a, MovingObject>,
    WriteStorage<'a, HasAABB>,
    FetchMut<'a, PlayerInput>,
    WriteStorage<'a, Renderable>,
);

struct PlayerAux;
impl PlayerAux {
    pub fn movement(mv: &mut MovingObject, bb: &HasAABB, direction: &Directional) {
        match *direction {
            Directional::Left => if bb.pushes_left_wall {
                PlayerAux::stop(mv);
            } else {
                mv.accel.x = -PC::WALK_ACCEL;
                mv.velocity.x = (-PC::WALK_SPEED / 2.0).min(mv.velocity.x).max(-PC::WALK_SPEED);
            },
            Directional::Right => if bb.pushes_right_wall {
                PlayerAux::stop(mv);
            } else {
                mv.accel.x = PC::WALK_ACCEL;
                mv.velocity.x = (PC::WALK_SPEED / 2.0).max(mv.velocity.x).min(PC::WALK_SPEED);
            },
        }
    }

    pub fn stop(mv: &mut MovingObject) {
        mv.accel.x = 0.0;
        mv.velocity.x = 0.0;
    }

    pub fn slow_down(mv: &mut MovingObject, fast: bool) {
        if fast {
            if mv.velocity.x > PC::WALK_ACCEL / 2.0 {
                mv.accel.x = -PC::WALK_ACCEL * 5.0;
            } else if mv.velocity.x < -PC::WALK_ACCEL / 2.0 {
                mv.accel.x = PC::WALK_ACCEL * 5.0;
            } else {
                PlayerAux::stop(mv);
            }
        } else {
            mv.accel.x = -mv.velocity.x / 2.0;
        }
    }
}
