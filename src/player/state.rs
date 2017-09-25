use components::*;
use player;
use player::animation_defs::*;
use player::consts as PC;
use player::state_machine::*;
use player::systems::PlayerAux;
use resources::*;

pub struct Idle;

impl State for Idle {
    fn on_start(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) {
        anim.sequence = PlayerAnimations::idle();
        rend.tpe.set_animation_id(player::P_IDLE, 10);
        // player.dj.enable();
    }

    fn on_resume(
        &mut self,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        dir: &Directional,
        pi: &PlayerInput,
        delta: &DeltaTime,
    ) {
        self.on_start(mv, bb, anim, rend, dir, pi, delta);
    }
    /// Executed on every frame before updating, for use in reacting to events.
    fn handle_events(
        &mut self,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        pi: &PlayerInput,
        _delta: &DeltaTime,
    ) -> Trans {
        let trans = if !bb.on_ground {
            anim.sequence = PlayerAnimations::drop();
            Trans::Push(Box::new(Jumping))
        } else if pi.jump {
            mv.velocity.y = PC::JUMP_SPEED;
            anim.sequence = PlayerAnimations::jump();
            Trans::Push(Box::new(Jumping))
        } else if pi.down && bb.on_platform {
            mv.position.y -= HumanoidMovement::PLATFORM_THRESHOLD;
            Trans::None
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

    fn update(
        &mut self,
        mv: &mut MovingObject,
        _bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) -> Trans {
        PlayerAux::slow_down(&mut *mv, true);
        Trans::None
    }
}

pub struct Running;

impl State for Running {
    fn on_start(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) {
        anim.sequence = PlayerAnimations::run();
        rend.tpe.set_animation_id(player::P_RUN, 10);
        // player.dj.enable();
    }

    fn on_resume(
        &mut self,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        dir: &Directional,
        pi: &PlayerInput,
        delta: &DeltaTime,
    ) {
        self.on_start(mv, bb, anim, rend, dir, pi, delta);
    }

    fn handle_events(
        &mut self,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        pi: &PlayerInput,
        _delta: &DeltaTime,
    ) -> Trans {
        if !(pi.left ^ pi.right) {
            return Trans::Switch(Box::new(Idle));
        };

        let trans = if !bb.on_ground {
            anim.sequence = PlayerAnimations::drop();
            Trans::Push(Box::new(Jumping))
        } else if pi.jump {
            mv.velocity.y = PC::JUMP_SPEED;
            anim.sequence = PlayerAnimations::jump();
            Trans::Push(Box::new(Jumping))
        } else if pi.down && bb.on_platform {
            mv.position.y -= HumanoidMovement::PLATFORM_THRESHOLD * 2.0;
            Trans::None
        } else if pi.slide {
            Trans::Push(Box::new(Sliding))
        } else if pi.attack {
            Trans::Push(Box::new(Attacking))
        } else {
            Trans::None
        };

        trans
    }

    fn update(
        &mut self,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) -> Trans {
        PlayerAux::movement(&mut *mv, &mut *bb, &dir);
        Trans::None
    }
}

pub struct Jumping;

impl State for Jumping {
    fn on_start(
        &mut self,
        _mv: &mut MovingObject,
        bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) {
        rend.tpe.set_animation_id(player::P_JUMP, 10);
        if !bb.on_ground && bb.was_on_ground {
            bb.frames_from_jump_start = 0;
        }
    }

    fn on_resume(
        &mut self,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        dir: &Directional,
        pi: &PlayerInput,
        delta: &DeltaTime,
    ) {
        self.on_start(mv, bb, anim, rend, dir, pi, delta)
    }

    fn handle_events(
        &mut self,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        dir: &Directional,
        pi: &PlayerInput,
        _delta: &DeltaTime,
    ) -> Trans {
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

    fn update(
        &mut self,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        pi: &PlayerInput,
        time: &DeltaTime,
    ) -> Trans {
        let y_vel = PC::GRAVITY * time.delta + mv.velocity.y;
        mv.velocity.y = y_vel.max(PC::MAX_FALLING_SPEED);
        // let gl = player.lg.grab_ledge(&mut mv, &pi, terrain);

        let trans = if bb.on_ground {
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
        };
        trans
    }

    fn fixed_update(
        &mut self,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) -> Trans {
        if bb.frames_from_jump_start <= PC::JUMP_FRAMES_THRESHOLD {
            if bb.at_ceiling || mv.velocity.y > 0.0 {
                bb.frames_from_jump_start = PC::JUMP_FRAMES_THRESHOLD + 1;
            }
        }

        bb.frames_from_jump_start += 1;
        Trans::None
    }
}

pub struct Sliding;

impl State for Sliding {
    fn on_start(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) {
        anim.sequence = PlayerAnimations::slide();
        rend.tpe.set_animation_id(player::P_SLIDE, 10);
    }

    fn handle_events(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        pi: &PlayerInput,
        _delta: &DeltaTime,
    ) -> Trans {
        let trans = if pi.jump {
            anim.sequence = PlayerAnimations::jump();
            Trans::Switch(Box::new(Jumping))
        } else {
            Trans::None
        };
        trans
    }

    fn update(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) -> Trans {
        Trans::None
    }

    fn fixed_update(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) -> Trans {
        if anim.sequence.is_over() {
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

impl State for Attacking {
    fn on_start(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) {
        rend.tpe.set_animation_id(player::P_ATTACK, 10);
        anim.sequence = PlayerAnimations::attack();
    }

    fn handle_events(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) -> Trans {
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

    fn fixed_update(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) -> Trans {
        if anim.sequence.is_over() {
            Trans::Pop
        } else {
            Trans::None
        }
    }

    fn update(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) -> Trans {
        Trans::None
    }
}

// pub struct LedgeGrab;

// impl State for LedgeGrab {
//     fn on_start(&mut self,   mv: &mut MovingObject,         bb: &mut HasAABB,         anim: &mut HasAnimationSequence,         rend: &mut Renderable,         dir: &Directional,         pi: &PlayerInput,         delta: &DeltaTime,) {
//         player.data.idle.reset();
//         player.dj.enable();
//     }
//     fn on_resume(&mut self,   mv: &mut MovingObject,         bb: &mut HasAABB,         anim: &mut HasAnimationSequence,         rend: &mut Renderable,         dir: &Directional,         pi: &PlayerInput,         delta: &DeltaTime,) {
//         self.on_start(player);
//     }
//     /// Executed on every frame before updating, for use in reacting to events.
//     fn handle_events(&mut self, _  mv: &mut MovingObject,         bb: &mut HasAABB,         anim: &mut HasAnimationSequence,         rend: &mut Renderable,         dir: &Directional,         pi: &PlayerInput,         delta: &DeltaTime,) -> Trans {
//         Trans::None
//     }

//     fn update(&mut self,   mv: &mut MovingObject,         bb: &mut HasAABB,         anim: &mut HasAnimationSequence,         rend: &mut Renderable,         dir: &Directional,         pi: &PlayerInput,         delta: &DeltaTime,, duration: &Duration, terrain: &Terrain) -> Trans {
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

//     fn fixed_update(&mut self,   mv: &mut MovingObject,         bb: &mut HasAABB,         anim: &mut HasAnimationSequence,         rend: &mut Renderable,         dir: &Directional,         pi: &PlayerInput,         delta: &DeltaTime,) -> Trans {
//         player.data.idle.roll_frames();
//         Trans::None
//     }

//     fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
//         draw_animation_frame(player, ctx, camera, &player.data.idle, &player.direction).unwrap();
//     }
// }
