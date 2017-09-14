use components::*;
use player::components::*;
use player::consts as PC;
use resources::*;
use specs::*;

pub struct PlayerDirectionSystem;
impl<'a> System<'a> for PlayerDirectionSystem {
    type SystemData = (WriteStorage<'a, Directional>,
     ReadStorage<'a, Controlled>,
     Fetch<'a, PlayerInput>);

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

type PlayerSystemData<'a> = (Entities<'a>,
                             ReadStorage<'a, Controlled>,
                             WriteStorage<'a, MovingObject>,
                             WriteStorage<'a, HasAABB>,
                             ReadStorage<'a, Directional>,
                             WriteStorage<'a, IdlePlayer>,
                             WriteStorage<'a, RunningPlayer>,
                             WriteStorage<'a, JumpingPlayer>,
                             WriteStorage<'a, SlidingPlayer>,
                             WriteStorage<'a, AttackingPlayer>,
                             Fetch<'a, PlayerInput>);

pub struct PlayerIdleControlSystem;
impl<'a> System<'a> for PlayerIdleControlSystem {
    type SystemData = PlayerSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (entities,
             ctrl,
             mut mv,
             bb,
             dir,
             mut idle,
             mut running,
             mut jumping,
             mut sliding,
             mut attacking,
             input) = data;
        let mut ss = |state, e| {
            switch_state(
                state,
                e,
                &mut idle,
                &mut running,
                &mut jumping,
                &mut sliding,
                &mut attacking,
            );
        };

        for (e, _, mut mv, bb) in (&*entities, &ctrl, &mut mv, &bb).join() {
            if idle.get(e).is_some() {
                if !bb.on_ground {
                    ss(PlayerState::Jumping, e);
                } else if input.jump {
                    mv.velocity.y = PC::JUMP_SPEED;
                    ss(PlayerState::Jumping, e);
                } else if input.down && bb.on_platform {
                    mv.position.y -= HumanoidMovement::PLATFORM_THRESHOLD * 2.0;
                    ss(PlayerState::Jumping, e);
                } else if input.left ^ input.right {
                    ss(PlayerState::Running, e);
                } else if input.slide {
                    ss(PlayerState::Sliding, e);
                } else if input.attack {
                    ss(PlayerState::Attacking, e);
                };
            }
        }
    }
}

pub struct PlayerRunningControlSystem;
impl<'a> System<'a> for PlayerRunningControlSystem {
    type SystemData = PlayerSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (entities,
             ctrl,
             mut mv,
             bb,
             dir,
             mut idle,
             mut running,
             mut jumping,
             mut sliding,
             mut attacking,
             input) = data;
        let mut ss = |state, e| {
            switch_state(
                state,
                e,
                &mut idle,
                &mut running,
                &mut jumping,
                &mut sliding,
                &mut attacking,
            );
        };

        for (e, _, mut mv, bb) in (&*entities, &ctrl, &mut mv, &bb).join() {
            if !(input.left ^ input.right) {
                ss(PlayerState::Idle, e);
            } else if input.jump {
                mv.velocity.y = PC::JUMP_SPEED;
                ss(PlayerState::Jumping, e);
            } else if input.down && bb.on_platform {
                mv.position.y -= HumanoidMovement::PLATFORM_THRESHOLD * 2.0;
                ss(PlayerState::Jumping, e);
            } else if input.slide {
                ss(PlayerState::Sliding, e);
            } else if input.attack {
                ss(PlayerState::Attacking, e);
            };
        }
    }
}

pub struct PlayerJumpingControlSystem;
impl<'a> System<'a> for PlayerJumpingControlSystem {
    type SystemData = PlayerSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (entities,
             ctrl,
             mut mv,
             bb,
             dir,
             mut idle,
             mut running,
             mut jumping,
             mut sliding,
             mut attacking,
             input) = data;
        let mut ss = |state, e| {
            switch_state(
                state,
                e,
                &mut idle,
                &mut running,
                &mut jumping,
                &mut sliding,
                &mut attacking,
            );
        };

        for (e, _, mut mv, mut bb, dir) in (&*entities, &ctrl, &mut mv, &mut bb, &dir).join() {

            if bb.cannot_go_left_frames > 0 {
                bb.cannot_go_left_frames -= 1;
                input.left = false;
            };

            if bb.cannot_go_right_frames > 0 {
                bb.cannot_go_right_frames -= 1;
                input.right = false;
            };

            if input.left ^ input.right {
                PlayerAux::movement(mv, bb, dir);
            };

            if input.attack {
                ss(PlayerState::Attacking, e);
            } else if input.jump {
                if bb.frames_from_jump_start <= PC::JUMP_FRAMES_THRESHOLD &&
                    mv.velocity.y <= 0.0 && !bb.at_ceiling
                {
                    mv.velocity.y = PC::JUMP_SPEED;
                } /*else {
                player.dj.double_jump(&mut player.mv);
                Trans::None
            }*/
            };
        }
    }
}

pub struct PlayerPP;



pub struct ResetActionsSystem;
impl<'a> System<'a> for ResetActionsSystem {
    type SystemData = FetchMut<'a, PlayerInput>;

    fn run(&mut self, input: Self::SystemData) {
        input.reset_actions();
    }
}

enum PlayerState {
    Idle,
    Running,
    Jumping,
    Sliding,
    Attacking,
}

fn switch_state<'a>(
    state: PlayerState,
    entity: Entity,
    idle: &mut WriteStorage<'a, IdlePlayer>,
    running: &mut WriteStorage<'a, RunningPlayer>,
    jumping: &mut WriteStorage<'a, JumpingPlayer>,
    sliding: &mut WriteStorage<'a, SlidingPlayer>,
    attacking: &mut WriteStorage<'a, AttackingPlayer>,
) {
    idle.remove(entity);
    running.remove(entity);
    jumping.remove(entity);
    sliding.remove(entity);
    attacking.remove(entity);

    match state {
        PlayerState::Idle => {
            idle.insert(entity, IdlePlayer);
        }
        PlayerState::Running => {
            running.insert(entity, RunningPlayer);
        }
        PlayerState::Jumping => {
            jumping.insert(entity, JumpingPlayer);
        }
        PlayerState::Sliding => {
            sliding.insert(entity, SlidingPlayer);
        }
        PlayerState::Attacking => {
            attacking.insert(entity, AttackingPlayer);
        }
    };
}


struct PlayerAux;
impl PlayerAux {
    pub fn movement(mv: &mut MovingObject, bb: &HasAABB, direction: &Directional) {
        match *direction {
            Directional::Left => {
                if bb.pushes_left_wall {
                    PlayerAux::stop(mv);
                } else {
                    mv.accel.x = -PC::WALK_ACCEL;
                    mv.velocity.x = (-PC::WALK_SPEED / 2.0).min(mv.velocity.x).max(-PC::WALK_SPEED);
                }
            }
            Directional::Right => {
                if bb.pushes_right_wall {
                    PlayerAux::stop(mv);
                } else {
                    mv.accel.x = PC::WALK_ACCEL;
                    mv.velocity.x = (PC::WALK_SPEED / 2.0).max(mv.velocity.x).min(PC::WALK_SPEED);
                }
            }
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
