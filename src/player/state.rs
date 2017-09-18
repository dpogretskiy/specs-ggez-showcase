use components::Directional;
use physics::components::*;
use player::animation_defs::*;
use player::consts as PC;
use player::state_machine::*;
use player::systems::PlayerAux;
use rendering::animation_seq::*;
use resources::*;
use std::cell::RefCell;
use util::seconds;

pub type PlayerData = (
    RefCell<MovingObject>,
    RefCell<HasAABB>,
    RefCell<HasAnimationSequence>,
    Directional,
    PlayerInput,
    DeltaTime,
);

pub struct Idle;

impl State<PlayerData> for Idle {
    fn on_start(&mut self, data: &mut PlayerData) {
        let &mut (_, _, ref anim, _, _, _) = data;
        anim.borrow_mut().sequence = PlayerAnimations::idle();
        println!("Idle now!");
        // player.dj.enable();
    }

    fn on_resume(&mut self, data: &mut PlayerData) {
        self.on_start(data);
    }
    /// Executed on every frame before updating, for use in reacting to events.
    fn handle_events(&mut self, data: &mut PlayerData) -> Trans<PlayerData> {
        let &mut (ref mv, ref bb, ref anim, _, ref pi, _) = data;
        let mut mv = mv.borrow_mut();
        let mut bb = bb.borrow_mut();

        let trans = if !bb.on_ground {
            anim.borrow_mut().sequence = PlayerAnimations::drop();
            Trans::Push(Box::new(Jumping))
        } else if pi.jump {
            mv.velocity.y = PC::JUMP_SPEED;
            anim.borrow_mut().sequence = PlayerAnimations::jump();
            Trans::Push(Box::new(Jumping))
        } else if pi.down {
            if bb.on_platform {
                mv.position.y -= HumanoidMovement::PLATFORM_THRESHOLD * 2.0;
            };
            anim.borrow_mut().sequence = PlayerAnimations::drop();
            Trans::Push(Box::new(Jumping))
        } else if pi.left ^ pi.right {
            Trans::Push(Box::new(Running))
        } else if pi.slide {
            Trans::Push(Box::new(Sliding))
        } else if pi.attack {
            Trans::Push(Box::new(Attacking))
        } else {
            Trans::None
        };
        trans
    }

    fn update(&mut self, data: &mut PlayerData) -> Trans<PlayerData> {
        let &mut (ref mv, _, _, _, _, _) = data;
        PlayerAux::slow_down(&mut *mv.borrow_mut(), true);
        Trans::None
    }
}

pub struct Running;

impl State<PlayerData> for Running {
    fn on_start(&mut self, data: &mut PlayerData) {
        let &mut (_, _, ref anim, _, _, _) = data;
        anim.borrow_mut().sequence = PlayerAnimations::run();
        println!("Running now!");
        // player.dj.enable();
    }

    fn on_resume(&mut self, data: &mut PlayerData) {
        self.on_start(data);
    }

    fn handle_events(&mut self, data: &mut PlayerData) -> Trans<PlayerData> {
        let &mut (ref mv, ref bb, ref anim, _, ref pi, _) = data;
        let mut mv = mv.borrow_mut();
        let mut bb = bb.borrow_mut();

        if !(pi.left ^ pi.right) {
            return Trans::Switch(Box::new(Idle));
        };

        let trans = if !bb.on_ground {
            anim.borrow_mut().sequence = PlayerAnimations::drop();
            Trans::Push(Box::new(Jumping))
        } else if pi.jump {
            mv.velocity.y = PC::JUMP_SPEED;
            anim.borrow_mut().sequence = PlayerAnimations::jump();
            Trans::Push(Box::new(Jumping))
        } else if pi.down {
            if bb.on_platform {
                mv.position.y -= HumanoidMovement::PLATFORM_THRESHOLD * 2.0;
            }
            anim.borrow_mut().sequence = PlayerAnimations::drop();
            Trans::Push(Box::new(Jumping))
        } else if pi.slide {
            Trans::Push(Box::new(Sliding))
        } else if pi.attack {
            Trans::Push(Box::new(Attacking))
        } else {
            Trans::None
        };

        trans
    }

    fn update(&mut self, data: &mut PlayerData) -> Trans<PlayerData> {
        let &mut (ref mv, ref bb, _, ref dir, _, _) = data;
        let mut mv = mv.borrow_mut();
        let mut bb = bb.borrow_mut();

        PlayerAux::movement(&mut *mv, &mut *bb, &dir);
        Trans::None
    }
}

pub struct Jumping;

impl State<PlayerData> for Jumping {
    fn on_start(&mut self, data: &mut PlayerData) {
        let &mut (_, ref bb, ref anim, _, _, _) = data;
        let mut bb = bb.borrow_mut();

        if !bb.on_ground && bb.was_on_ground {
            bb.frames_from_jump_start = 0;
        }
        println!("Jumping now");
    }

    fn on_resume(&mut self, data: &mut PlayerData) {
        self.on_start(data)
    }

    fn handle_events(&mut self, data: &mut PlayerData) -> Trans<PlayerData> {
        let &mut (ref mv, ref bb, _, ref dir, ref pi, _) = data;
        let mut mv = mv.borrow_mut();
        let mut bb = bb.borrow_mut();

        let mut no_left = false;
        if bb.cannot_go_left_frames > 0 {
            bb.cannot_go_left_frames -= 1;
            no_left = true;
        };

        let mut no_right = false;
        if bb.cannot_go_right_frames > 0 {
            bb.cannot_go_right_frames -= 1;
            no_right = true;
        };

        if (pi.left && !no_left) ^ (pi.right && !no_right) {
            PlayerAux::movement(&mut *mv, &mut *bb, &dir);
        };

        let trans = if pi.attack {
            Trans::Switch(Box::new(Attacking))
        } else if pi.jump {
            if bb.frames_from_jump_start <= PC::JUMP_FRAMES_THRESHOLD && mv.velocity.y <= 0.0 &&
                !bb.at_ceiling
            {
                mv.velocity.y = PC::JUMP_SPEED;
                Trans::None
            } else {
                // player.dj.double_jump(&mut mv);
                Trans::None
            }
        } else {
            Trans::None
        };
        trans
    }

    fn update(&mut self, data: &mut PlayerData) -> Trans<PlayerData> {
        let &mut (ref mv, ref bb, _, _, ref pi, ref time) = data;
        let mut mv = mv.borrow_mut();
        let mut bb = bb.borrow_mut();

        let y_vel = PC::GRAVITY * seconds(&time.time) + mv.velocity.y;
        mv.velocity.y = y_vel.max(PC::MAX_FALLING_SPEED);
        // let gl = player.lg.grab_ledge(&mut mv, &pi, terrain);

        if bb.on_ground {
            Trans::Pop
        } else
        /*if gl {
            Trans::Switch(Box::new(LedgeGrab))
        } else*/
        if !(pi.left ^ pi.right) {
            PlayerAux::slow_down(&mut *mv, false);
            Trans::None
        } else {
            Trans::None
        }
    }

    fn fixed_update(&mut self, data: &mut PlayerData) -> Trans<PlayerData> {
        let &mut (ref mv, ref bb, _, _, _, _) = data;
        let mut bb = bb.borrow_mut();

        if bb.frames_from_jump_start <= PC::JUMP_FRAMES_THRESHOLD {
            if bb.at_ceiling || mv.borrow().velocity.y > 0.0 {
                bb.frames_from_jump_start = PC::JUMP_FRAMES_THRESHOLD + 1;
            }
        }

        bb.frames_from_jump_start += 1;
        Trans::None
    }
}

pub struct Sliding;

impl State<PlayerData> for Sliding {
    fn on_start(&mut self, data: &mut PlayerData) {
        let &mut (_, _, ref anim, _, _, _) = data;
        anim.borrow_mut().sequence = PlayerAnimations::slide();
        println!("sliding!");
    }

    fn handle_events(&mut self, data: &mut PlayerData) -> Trans<PlayerData> {
        let &mut (_, _, ref anim, _, ref pi, _) = data;
        let trans = if pi.jump {
            anim.borrow_mut().sequence = PlayerAnimations::jump();
            Trans::Switch(Box::new(Jumping))
        } else {
            Trans::None
        };
        trans
    }

    fn update(&mut self, data: &mut PlayerData) -> Trans<PlayerData> {
        Trans::None
    }

    fn fixed_update(&mut self, data: &mut PlayerData) -> Trans<PlayerData> {
        let &mut (_, _, ref anim, _, _, _) = data;
        if anim.borrow_mut().sequence.is_over() {
            Trans::Pop
        } else {
            Trans::None
        }
    }
}

pub struct Attacking;

// impl Attacking {
//     fn can_cancel(&self, player: &Player) -> bool {
//         player.data.attacking.current_frame > 5
//     }
// }

impl State<PlayerData> for Attacking {
    fn on_start(&mut self, data: &mut PlayerData) {
        let &mut (_, _, ref anim, _, _, _) = data;
        anim.borrow_mut().sequence = PlayerAnimations::attack();
        println!("Attack!");
    }

    fn handle_events(&mut self, data: &mut PlayerData) -> Trans<PlayerData> {
        let &mut (_, _, _, _, ref pi, _) = data;
        // let t = if self.can_cancel(player) {
        //     if pi.jump {
        //         Trans::Switch(Box::new(Jumping))
        //     } else if pi.slide {
        //         Trans::Switch(Box::new(Sliding))
        //     } else if pi.attack {
        //         Trans::Switch(Box::new(Attacking))
        //     } else {
        //         Trans::None
        //     }
        // } else {
        //     Trans::None
        // };

        Trans::None
    }

    fn fixed_update(&mut self, data: &mut PlayerData) -> Trans<PlayerData> {
        let &mut (_, _, ref anim, _, _, _) = data;
        if anim.borrow_mut().sequence.is_over() {
            Trans::Pop
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: &mut PlayerData) -> Trans<PlayerData> {
        let &mut (_, _, _, _, _, _) = data;
        Trans::None
    }
}

// pub struct LedgeGrab;

// impl State for LedgeGrab {
//     fn on_start(&mut self,  data: &mut PlayerData) {
//         player.data.idle.reset();
//         player.dj.enable();
//     }
//     fn on_resume(&mut self,  data: &mut PlayerData) {
//         self.on_start(player);
//     }
//     /// Executed on every frame before updating, for use in reacting to events.
//     fn handle_events(&mut self, _ data: &mut PlayerData) -> Trans {
//         Trans::None
//     }

//     fn update(&mut self,  data: &mut PlayerData, duration: &Duration, terrain: &Terrain) -> Trans {
//         mv.update_physics(duration, terrain);

//         let ledge_on_left =
//             player.lg.ledge_tile.0 as f64 * terrain.tile_size < mv.position.x;
//         let ledge_on_right = !ledge_on_left;

//         let state = if pi.down || (pi.right && ledge_on_left) ||
//             (pi.left && ledge_on_right)
//         {
//             if ledge_on_left {
//                 mv.cannot_go_left_frames = 3;
//             } else {
//                 mv.cannot_go_right_frames = 3;
//             };
//             Trans::Switch(Box::new(Jumping))
//         } else if pi.jump {
//             mv.velocity.y = Player::JUMP_SPEED;
//             Trans::Switch(Box::new(Jumping))
//         } else {
//             Trans::None
//         };

//         pi.reset_actions();
//         state
//     }

//     fn fixed_update(&mut self,  data: &mut PlayerData) -> Trans {
//         player.data.idle.roll_frames();
//         Trans::None
//     }

//     fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
//         draw_animation_frame(player, ctx, camera, &player.data.idle, &player.direction).unwrap();
//     }
// }
