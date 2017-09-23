use asset_storage::*;
use camera::*;
use components::*;
use ggez::Context;
use ggez::graphics::*;
use rayon::iter::ParallelIterator;
use specs::*;
use std::collections::BTreeMap;
use util::Vector2;

pub use physics::systems::*;
pub use player::systems::*;

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
            let mut scale: Scalable = scalable.get(e).unwrap_or_else(|| &default_scale).clone();

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
                                        dest: Point2::new(pos.x, pos.y),
                                        src: frame,
                                        scale: Point2::new(scale.x, scale.y),
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
                                    dest: Point2::new(pos.x, pos.y),
                                    scale: Point2::new(scale.x, scale.y),
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
                                    dest: Point2::new(pos.x, pos.y),
                                    scale: Point2::new(scale.x, scale.y),
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
    type SystemData = (FetchMut<'a, Camera>, ReadStorage<'a, Position>, ReadStorage<'a, SnapCamera>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut camera, position, snap) = data;

        for (p, _) in (&position, &snap).join() {
            camera.move_to(Vector2::new(p.x as f64, p.y as f64));
        }
    }
}

pub struct ChaseCameraSystem;
impl<'a> System<'a> for ChaseCameraSystem {
    type SystemData = (Fetch<'a, Camera>, ReadStorage<'a, ChaseCamera>, WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (cam, chase, mut pos) = data;

        for (pos, _) in (&mut pos, &chase).join() {
            let loc = cam.location();
            pos.x = loc.x as f32;
            pos.y = loc.y as f32;
        }
    }
}

pub struct AnimationFFSystem;
impl<'a> System<'a> for AnimationFFSystem {
    type SystemData = (WriteStorage<'a, HasAnimationSequence>, WriteStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut anim, mut rend) = data;

        (&mut anim, &mut rend).par_join().for_each(|(anim, rend)| match rend.tpe {
            RenderableType::Animation {
                ref id,
                ref mut frame,
                ref length,
            } => {
                if let Some(next) = anim.sequence.next() {
                    *frame = next;
                }
            }
            _ => (),
        });
    }
}
