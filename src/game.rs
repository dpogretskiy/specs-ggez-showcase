use ggez::*;
use ggez::event::*;
use level::*;

use asset_storage::*;
use camera::*;
use components::*;
use player::*;
use resources::*;
use specs::*;
use std::time::Duration;
use systems::*;

use util::{Vector2, seconds};

pub struct Game<'a, 'b> {
    pub world: World,
    pub player_count: usize,
    pub dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(ctx: &mut Context) -> GameResult<Game<'a, 'b>> {
        let mut world = World::new();
        let mut pc = 0;
        register_components(&mut world);

        //load everything!
        {
            let mut asset_storage = AssetStorage::empty();

            //level part
            {
                let level = Level::load(ctx, LevelType::Graveyard)?;
                let RenderableLevel {
                    background,
                    ground_batch,
                    objects_batch,
                    terrain,
                } = RenderableLevel::build(level);
                asset_storage.images.insert("level-background", background);
                asset_storage.batches.insert("level-ground", ground_batch);
                asset_storage.batches.insert("level-objects", objects_batch);
                world.add_resource(LevelTerrain { terrain });
            }
            //player part
            {
                AnimationLoader::load_assets(ctx, &mut asset_storage)?;
            }
            world.add_resource::<AssetStorage>(asset_storage);
        }

        world
            .create_entity()
            .with(Position::new(0.0, 0.0))
            .with(Renderable {
                layer: 1,
                tpe: RenderableType::Batch { id: "level-ground" },
            })
            .build();

        world
            .create_entity()
            .with(Position::new(0.0, 0.0))
            .with(Renderable {
                layer: 0,
                tpe: RenderableType::Image { id: "level-background" },
            })
            .with(Scalable::new(2.0, 2.0))
            .with(ChaseCamera)
            .build();

        world.add_resource(MousePointer(0.0, 0.0));
        world.add_resource(DeltaTime { delta: 0.0 });
        world.add_resource(PlayerInput::new());

        let (w, h) = (ctx.conf.window_width, ctx.conf.window_height);
        let hc = h as f64 / w as f64;
        let fov = w as f64 * 1.5;

        world.add_resource(Camera::new(w, h, fov, hc * fov));

        Player::spawn(&mut world, Vector2::new(500.0, 500.0), true, true, &mut pc);

        let dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new()
            .add(StartPSMSystem, "start-state-machines", &[])
            .add(PlayerDirectionSystem, "p.direct", &[])
            .add(
                PlayerHandleEventsSystem,
                "p.handle_events",
                &["p.direct"],
            )
            // .add(MovingSystem, "moving", &[])
            .add(AABBMovingSystem, "has_aabb", &[])
            .add(
                PlayerUpdateSystem,
                "p.update",
                &["p.handle_events"],
            )
            .add(PositionSystem, "position", &["has_aabb"])
            .add(
                ResetInputSystem,
                "p.reset_input",
                &["p.handle_events"],
            )
            .add(CollisionSystem, "collisions", &["has_aabb"])
            .add(CameraSnapSystem, "camera_snap", &["position"])
            .add(ChaseCameraSystem, "chase_camera", &["position"])
            .build();

        Ok(Game {
            world,
            player_count: pc,
            dispatcher,
        })
    }
}

impl<'a, 'b> event::EventHandler for Game<'a, 'b> {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        if timer::get_ticks(ctx) % 100 == 0 {
            println!("FPS: {}", timer::get_fps(ctx));
        }

        self.world.write_resource::<DeltaTime>().delta = seconds(&dt);

        if timer::check_update_time(ctx, 30) {
            PlayerFixedUpdateSystem.run_now(&mut self.world.res);
            AnimationFFSystem.run_now(&mut self.world.res);
        }

        self.dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        {
            let mut rs = RenderingSystem::new(ctx);
            rs.run_now(&mut self.world.res);
        }

        graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool) {
        let mut input = self.world.write_resource::<PlayerInput>();

        if !repeat {
            match keycode {
                Keycode::Left => input.left = true,
                Keycode::Right => input.right = true,
                Keycode::Up => input.up = true,
                Keycode::Down => input.down = true,
                Keycode::LCtrl => input.slide = true,
                Keycode::Space => input.jump = true,
                Keycode::LShift => input.attack = true,
                _ => (),
            }
        }
    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool) {
        let mut input = self.world.write_resource::<PlayerInput>();
        if !repeat {
            //wat?
            match keycode {
                Keycode::Left => input.left = false,
                Keycode::Right => input.right = false,
                Keycode::Up => input.up = false,
                Keycode::Down => input.down = false,
                _ => (),
            }
        }
    }

    fn controller_button_down_event(&mut self, btn: Button, _instance_id: i32) {
        let mut input = self.world.write_resource::<PlayerInput>();
        match btn {
            Button::A => input.jump = true,
            Button::X => input.attack = true,
            Button::B => input.slide = true,
            // Button::LeftShoulder => self.player.mv.position = Vector2::new(300.0, 500.0),
            _ => (),
        }
    }
    fn controller_button_up_event(&mut self, _btn: Button, _instance_id: i32) {}
    fn controller_axis_event(&mut self, axis: Axis, value: i16, _instance_id: i32) {
        let mut input = self.world.write_resource::<PlayerInput>();
        match axis {
            Axis::LeftX => {
                if value > 7500 {
                    input.right = true
                } else {
                    input.right = false
                };
                if value < -7500 {
                    input.left = true
                } else {
                    input.left = false
                }
            }
            Axis::LeftY => {
                if value > 7500 {
                    input.down = true
                } else {
                    input.down = false
                }
            }
            _ => (),
        }
    }

    fn mouse_button_down_event(&mut self, button: event::MouseButton, x: i32, y: i32) {
        if button == event::MouseButton::Left {
            let p = self.world.read_resource::<Camera>().screen_to_world_coords((x, y));
            Player::spawn(&mut self.world, p, false, false, &mut self.player_count)
        }
    }

    fn mouse_motion_event(&mut self, _state: MouseState, x: i32, y: i32, _: i32, _: i32) {
        let coords = self.world.read_resource::<Camera>().screen_to_world_coords((x, y));
        let mut mp = self.world.write_resource::<MousePointer>();
        mp.0 = coords.x;
        mp.1 = coords.y;
    }

    fn mouse_wheel_event(&mut self, _: i32, _: i32) {
        let mp = self.world.read_resource::<MousePointer>().clone();
        let p = Vector2::new(mp.0, mp.1);
        Player::spawn(&mut self.world, p, false, false, &mut self.player_count);
    }
}
