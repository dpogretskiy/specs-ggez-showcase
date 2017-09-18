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
use player;

use player::animation_defs::PlayerAnimations;
use physics::AABB;


use util::Vector2;

pub struct Game<'a, 'b> {
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(ctx: &mut Context) -> GameResult<Game<'a, 'b>> {
        let mut world = World::new();

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
                PlayerLoader::load_assets(ctx, &mut asset_storage)?;
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
                tpe: RenderableType::Image {
                    id: "level-background",
                },
            })
            .with(Scalable::new(2.0, 2.0))
            .build();

        let mut psm = PlayerStateMachine { machine: state_machine::StateMachine::new(state::Idle) };

        world.create_entity()
            .with(Position::new(0.0, 0.0))
            .with(Renderable{ layer: 5, tpe: RenderableType::Animation { id: "player-idle", frame: 0, length: 10 }})
            .with(HasAnimationSequence { sequence: PlayerAnimations::idle() })
            .with(Controlled)
            .with(psm)
            .with(StartPSM)
            .with(SnapCamera)
            .with(Directional::Right)
            .with(Scalable::new(0.4, 0.4))
            .with(MovingObject::new(Vector2::new(300.0, 500.0)))
            .with(HasAABB::new(AABB::new_full(Vector2::new(300.0, 500.0), Vector2::new(290.0, 500.0), Vector2::new(0.4, 0.4))))
            .build();

        world.add_resource(DeltaTime {
            time: Duration::from_secs(0),
        });
        world.add_resource(PlayerInput::new());

        let (w, h) = (ctx.conf.window_width, ctx.conf.window_height);
        let hc = h as f64 / w as f64;
        let fov = w as f64 * 1.5;
        world.add_resource(Camera::new(w, h, fov, hc * fov));

        let dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new()
            .add(StartPSMSystem, "start-state-machines", &[])
            .add(PlayerDirectionSystem, "player.direct", &[])
            .add(
                PlayerHandleEventsSystem,
                "player_sm.handle_events",
                &["player.direct"],
            )
            .add(
                PlayerUpdateSystem,
                "player_sm.update",
                &["player_sm.handle_events"],
            )
            .add(MovingSystem, "moving", &["player_sm.update"])
            .add(HasAABBSystem, "has_aabb", &["moving"])
            .add(PositionSystem, "position", &["moving", "has_aabb"])
            .add(CameraSnapSystem, "camera_snap", &["position"])
            .build();

        Ok(Game { world, dispatcher })
    }
}

impl<'a, 'b> event::EventHandler for Game<'a, 'b> {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        if timer::get_ticks(ctx) % 1000 == 0 {
            println!("FPS: {}", timer::get_fps(ctx));
        }

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
            Axis::LeftY => if value > 7500 {
                input.down = true
            } else {
                input.down = false
            },
            _ => (),
        }
    }

    fn mouse_button_down_event(&mut self, button: event::MouseButton, x: i32, y: i32) {
        if button == event::MouseButton::Left {
            // let p = self.camera.screen_to_world_coords((x, y));
            // let rect = graphics::Rect::new(0.0, 0.41133004, 0.25728154, 0.26108375);
            // let rect = graphics::Rect::new(0.0, 0.0, 1.0, 1.0);

            // self.boxes.push(GameBox::new(
            //     p,
            //     &self.level.objects.image,
            //     rect,
            // ));
        }
    }
}
