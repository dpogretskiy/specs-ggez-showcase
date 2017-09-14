use super::state_machine::*;
use super::systems::PlayerSystemData;
pub struct Idle;

// impl State<PlayerSystemData<'a>> for Idle {
//     fn on_start(&mut self, data: &mut PlayerSystemData<'a>) {
//         player.data.idle.reset();
//         player.dj.enable();
//     }
//     fn on_resume(&mut self, player: &mut Player) {
//         self.on_start(player);
//     }
//     /// Executed on every frame before updating, for use in reacting to events.
//     fn handle_events(&mut self, player: &mut Player) -> Trans {
//         player.direct();

//         let pi = &mut player.input;
//         let mv = &mut player.mv;

//         let trans = if !mv.on_ground {
//             Trans::Push(Box::new(Jumping))
//         } else if pi.jump {
//             mv.velocity.y = Player::JUMP_SPEED;
//             Trans::Push(Box::new(Jumping))
//         } else if pi.down {
//             if mv.on_platform {
//                 mv.position.y -= MovingObject::PLATFORM_THRESHOLD * 2.0;
//             };
//             Trans::Push(Box::new(Jumping))
//         } else if pi.left ^ pi.right {
//             Trans::Push(Box::new(Running))
//         } else if pi.slide {
//             Trans::Push(Box::new(Sliding))
//         } else if pi.attack {
//             Trans::Push(Box::new(Attacking))
//         } else {
//             Trans::None
//         };

//         pi.reset_actions();
//         trans
//     }

//     fn update(&mut self, player: &mut Player, duration: &Duration, terrain: &Terrain) -> Trans {
//         player.slow_down(true);
//         player.mv.update_physics(duration, terrain);
//         Trans::None
//     }

//     fn fixed_update(&mut self, player: &mut Player) -> Trans {
//         player.data.idle.roll_frames();
//         Trans::None
//     }

//     fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
//         draw_animation_frame(player, ctx, camera, &player.data.idle, &player.direction).unwrap();
//     }
// }

// pub struct Running;

// impl State for Running {
//     fn on_start(&mut self, player: &mut Player) {
//         player.data.running.reset();
//         player.dj.enable();
//     }
//     fn on_resume(&mut self, player: &mut Player) {
//         self.on_start(player);
//     }
//     fn on_pause(&mut self, _player: &mut Player) {}
//     fn on_stop(&mut self, _player: &mut Player) {}

//     fn handle_events(&mut self, player: &mut Player) -> Trans {
//         player.direct();
//         let pi = &mut player.input;
//         let mv = &mut player.mv;

//         if !(pi.left ^ pi.right) {
//             return Trans::Switch(Box::new(Idle));
//         };

//         let t = if !mv.on_ground {
//             Trans::Push(Box::new(Jumping))
//         } else if pi.jump {
//             mv.velocity.y = Player::JUMP_SPEED;
//             Trans::Push(Box::new(Jumping))
//         } else if pi.down {
//             if mv.on_platform {
//                 mv.position.y -= MovingObject::PLATFORM_THRESHOLD * 2.0;
//             }
//             Trans::Push(Box::new(Jumping))
//         } else if pi.slide {
//             Trans::Push(Box::new(Sliding))
//         } else if pi.attack {
//             Trans::Push(Box::new(Attacking))
//         } else {
//             Trans::None
//         };

//         pi.reset_actions();
//         t
//     }

//     fn update(&mut self, player: &mut Player, duration: &Duration, terrain: &Terrain) -> Trans {
//         player.movement();
//         player.mv.update_physics(duration, terrain);
//         Trans::None
//     }

//     fn fixed_update(&mut self, player: &mut Player) -> Trans {
//         player.data.running.cycle_frames();
//         Trans::None
//     }

//     fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
//         draw_animation_frame(player, ctx, camera, &player.data.running, &player.direction).unwrap();
//     }
// }

// pub struct Jumping;

// impl State for Jumping {
//     fn on_start(&mut self, player: &mut Player) {
//         player.data.jumping.reset();

//         if !player.mv.on_ground && player.mv.was_on_ground {
//             player.mv.frames_from_jump_start = 0;
//         }
//     }

//     fn on_resume(&mut self, player: &mut Player) {
//         self.on_start(player)
//     }

//     fn handle_events(&mut self, player: &mut Player) -> Trans {
//         player.direct();

//         if player.mv.cannot_go_left_frames > 0 {
//             player.mv.cannot_go_left_frames -= 1;
//             player.input.left = false;
//         };

//         if player.mv.cannot_go_right_frames > 0 {
//             player.mv.cannot_go_right_frames -= 1;
//             player.input.right = false;
//         };

//         if player.input.left ^ player.input.right {
//             player.movement();
//         };

//         let t = if player.input.attack {
//             Trans::Switch(Box::new(Attacking))
//         } else if player.input.jump {
//             if player.mv.frames_from_jump_start <= Player::JUMP_FRAMES_THRESHOLD &&
//                 player.mv.velocity.y <= 0.0 && !player.mv.at_ceiling
//             {
//                 player.mv.velocity.y = Player::JUMP_SPEED;
//                 Trans::None
//             } else {
//                 player.dj.double_jump(&mut player.mv);
//                 Trans::None
//             }
//         } else {
//             Trans::None
//         };

//         player.input.reset_actions();
//         t
//     }

//     fn update(&mut self, player: &mut Player, duration: &Duration, terrain: &Terrain) -> Trans {
//         let y_vel = Player::GRAVITY * seconds(&duration) + player.mv.velocity.y;
//         player.mv.velocity.y = y_vel.max(Player::MAX_FALLING_SPEED);
//         player.mv.update_physics(duration, terrain);
//         let gl = player.lg.grab_ledge(&mut player.mv, &player.input, terrain);

//         if player.mv.on_ground {
//             Trans::Pop
//         } else if gl {
//             Trans::Switch(Box::new(LedgeGrab))
//         } else if !(player.input.left ^ player.input.right) {
//             player.slow_down(false);
//             Trans::None
//         } else {
//             Trans::None
//         }
//     }

//     fn fixed_update(&mut self, player: &mut Player) -> Trans {
//         player.data.jumping.roll_frames();
//         if player.mv.frames_from_jump_start <= Player::JUMP_FRAMES_THRESHOLD {
//             if player.mv.at_ceiling || player.mv.velocity.y > 0.0 {
//                 player.mv.frames_from_jump_start = Player::JUMP_FRAMES_THRESHOLD + 1;
//             }
//         }

//         player.mv.frames_from_jump_start += 1;
//         Trans::None
//     }

//     fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
//         draw_animation_frame(player, ctx, camera, &player.data.jumping, &player.direction).unwrap();
//     }
// }

// pub struct Sliding;

// impl State for Sliding {
//     fn on_start(&mut self, player: &mut Player) {
//         player.data.sliding.reset();
//     }

//     fn handle_events(&mut self, player: &mut Player) -> Trans {
//         let t = if player.input.jump {
//             Trans::Switch(Box::new(Jumping))
//         } else {
//             Trans::None
//         };

//         player.input.reset_actions();
//         t
//     }

//     fn update(&mut self, player: &mut Player, duration: &Duration, terrain: &Terrain) -> Trans {
//         player.mv.update_physics(duration, terrain);
//         Trans::None
//     }

//     fn fixed_update(&mut self, player: &mut Player) -> Trans {
//         if player.data.sliding.is_over() {
//             Trans::Pop
//         } else {
//             player.data.sliding.next_frame();
//             Trans::None
//         }
//     }

//     fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
//         draw_animation_frame(player, ctx, camera, &player.data.sliding, &player.direction).unwrap();
//     }
// }

// pub struct Attacking;

// impl Attacking {
//     fn can_cancel(&self, player: &Player) -> bool {
//         player.data.attacking.current_frame > 5
//     }
// }

// impl State for Attacking {
//     fn on_start(&mut self, player: &mut Player) {
//         player.data.attacking.reset();
//     }

//     fn handle_events(&mut self, player: &mut Player) -> Trans {
//         player.direct();

//         let t = if self.can_cancel(player) {
//             if player.input.jump {
//                 Trans::Switch(Box::new(Jumping))
//             } else if player.input.slide {
//                 Trans::Switch(Box::new(Sliding))
//             } else if player.input.attack {
//                 Trans::Switch(Box::new(Attacking))
//             } else {
//                 Trans::None
//             }
//         } else {
//             Trans::None
//         };

//         player.input.reset_actions();
//         t
//     }

//     fn fixed_update(&mut self, player: &mut Player) -> Trans {
//         if player.data.attacking.is_over() {
//             Trans::Pop
//         } else {
//             player.data.attacking.next_frame();
//             Trans::None
//         }
//     }

//     fn update(&mut self, player: &mut Player, duration: &Duration, terrain: &Terrain) -> Trans {
//         player.mv.update_physics(duration, terrain);
//         Trans::None
//     }

//     fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
//         draw_animation_frame(
//             player,
//             ctx,
//             &camera,
//             &player.data.attacking,
//             &player.direction,
//         ).unwrap();
//     }
// }

// pub struct LedgeGrab;

// impl State for LedgeGrab {
//     fn on_start(&mut self, player: &mut Player) {
//         player.data.idle.reset();
//         player.dj.enable();
//     }
//     fn on_resume(&mut self, player: &mut Player) {
//         self.on_start(player);
//     }
//     /// Executed on every frame before updating, for use in reacting to events.
//     fn handle_events(&mut self, _player: &mut Player) -> Trans {
//         Trans::None
//     }

//     fn update(&mut self, player: &mut Player, duration: &Duration, terrain: &Terrain) -> Trans {
//         player.mv.update_physics(duration, terrain);

//         let ledge_on_left =
//             player.lg.ledge_tile.0 as f64 * terrain.tile_size < player.mv.position.x;
//         let ledge_on_right = !ledge_on_left;

//         let state = if player.input.down || (player.input.right && ledge_on_left) ||
//             (player.input.left && ledge_on_right)
//         {
//             if ledge_on_left {
//                 player.mv.cannot_go_left_frames = 3;
//             } else {
//                 player.mv.cannot_go_right_frames = 3;
//             };
//             Trans::Switch(Box::new(Jumping))
//         } else if player.input.jump {
//             player.mv.velocity.y = Player::JUMP_SPEED;
//             Trans::Switch(Box::new(Jumping))
//         } else {
//             Trans::None
//         };

//         player.input.reset_actions();
//         state
//     }

//     fn fixed_update(&mut self, player: &mut Player) -> Trans {
//         player.data.idle.roll_frames();
//         Trans::None
//     }

//     fn draw(&mut self, ctx: &mut Context, player: &Player, camera: &Camera) {
//         draw_animation_frame(player, ctx, camera, &player.data.idle, &player.direction).unwrap();
//     }
// }
