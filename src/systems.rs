use asset_storage::*;
use camera::*;
use components::*;
use ggez::Context;
use ggez::graphics::*;
use specs::*;
use std::collections::BTreeMap;
use util::Vector2;

pub use physics::systems::*;

pub struct RenderingSystem<'c> {
    ctx: &'c mut Context,
}

impl<'c> RenderingSystem<'c> {
    pub fn new(ctx: &'c mut Context) -> RenderingSystem<'c> {
        RenderingSystem { ctx }
    }
}

impl<'a, 'c> System<'a> for RenderingSystem<'c> {
    type SystemData = (Entities<'a>,
     Fetch<'a, AssetStorage>,
     Fetch<'a, Camera>,
     ReadStorage<'a, Renderable>,
     ReadStorage<'a, Position>,
     ReadStorage<'a, Scalable>,
     ReadStorage<'a, Directional>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, assets, camera, renderable, position, scalable, directional) = data;

        let default_scale = Scalable::new(1.0, 1.0);

        let mut layers = BTreeMap::new();

        for (e, r, pos) in (&*entities, &renderable, &position).join() {
            let mut scale = scalable.get(e).unwrap_or_else(|| &default_scale).clone();

            if let Some(&Directional::Left) = directional.get(e) {
                scale.x = -scale.x;
            }

            layers
                .entry(r.layer)
                .or_insert(vec![(r.tpe.clone(), pos.clone(), scale)])
                .push((r.tpe.clone(), pos.clone(), scale));
        }

        for (_, data) in layers.into_iter() {
            for (rt, pos, scale) in data.into_iter() {
                match rt {
                    RenderableType::Animation { id, frame, length } => {
                        if let Some(a) = assets.animations.get(id) {
                            if frame < length {
                                let i = &a.image;
                                let frame = a.frames[frame];
                                i.draw_ex_camera(
                                    &*camera,
                                    self.ctx,
                                    DrawParam {
                                        dest: Point::new(pos.x, pos.y),
                                        src: frame,
                                        scale: Point::new(scale.x, scale.y),
                                        ..Default::default()
                                    },
                                ).unwrap();
                            }
                        }
                    }
                    RenderableType::Image { id } => {
                        if let Some(i) = assets.images.get(id) {
                            i.draw_ex_camera(
                                &*camera,
                                self.ctx,
                                DrawParam {
                                    dest: Point::new(pos.x, pos.y),
                                    scale: Point::new(scale.x, scale.y),
                                    ..Default::default()
                                },
                            ).unwrap();
                        }
                    }
                    RenderableType::Batch { id } => {
                        if let Some(b) = assets.batches.get(id) {
                            b.draw_ex_camera(
                                &*camera,
                                self.ctx,
                                DrawParam {
                                    dest: Point::new(pos.x, pos.y),
                                    scale: Point::new(scale.x, scale.y),
                                    ..Default::default()
                                },
                            ).unwrap();
                        }
                    }
                }
            }
        }
    }
}

pub struct CameraSnapSystem;
impl<'a> System<'a> for CameraSnapSystem {
    type SystemData = (FetchMut<'a, Camera>, ReadStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut camera, position) = data;

        for p in position.join() {
            camera.move_to(Vector2::new(p.x as f64, p.y as f64));
        }
    }
}
