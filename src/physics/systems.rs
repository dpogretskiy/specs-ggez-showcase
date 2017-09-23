use components::Position;
use physics::components::*;
use rayon::iter::ParallelIterator;
use resources::*;
use specs::*;
use util::*;

// pub struct MovingSystem;
// impl<'a> System<'a> for MovingSystem {
//     type SystemData = (WriteStorage<'a, MovingObject>, Fetch<'a, DeltaTime>);

//     fn run(&mut self, data: Self::SystemData) {
//         let (mut objects, time) = data;
//         let delta = seconds(&time.time);

//         (&mut objects).par_join().for_each(|o| {
//             o.old_position = o.position;
//             o.old_velocity = o.velocity;
//             o.old_accel = o.accel;
//             o.velocity += o.accel * delta;
//             o.position += o.velocity * delta;
//         });
//     }
// }

pub struct AABBMovingSystem;
impl<'a> System<'a> for AABBMovingSystem {
    type SystemData = (WriteStorage<'a, HasAABB>,
     WriteStorage<'a, MovingObject>,
     Fetch<'a, LevelTerrain>,
     Fetch<'a, DeltaTime>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut has_aabb, mut mv, level, delta) = data;
        let terrain = &level.terrain;
        let delta = seconds(&delta.time);

        (&mut has_aabb, &mut mv).par_join().for_each(|(bb, mv)| {
            mv.old_position = mv.position;
            mv.old_velocity = mv.velocity;
            mv.old_accel = mv.accel;
            mv.velocity += mv.accel * delta;
            mv.position += mv.velocity * delta;

            bb.was_on_platform = bb.on_platform;
            bb.was_on_ground = bb.on_ground;
            bb.was_at_ceiling = bb.at_ceiling;
            bb.pushed_left_wall = bb.pushes_left_wall;
            bb.pushed_right_wall = bb.pushes_right_wall;

            let mut ground_y = 0.0;
            let mut ceiling_y = 0.0;
            let mut right_wall_x = 0.0;
            let mut left_wall_x = 0.0;

            bb.on_platform = false;

            if mv.velocity.y <= 0.0 &&
                HumanoidMovement::has_ground(mv, bb, &mut ground_y, terrain)
            {
                mv.position.y = ground_y + bb.aabb.half_size.y - bb.aabb.offset.y;
                mv.velocity.y = 0.0;
                bb.on_ground = true;
            } else {
                bb.on_ground = false;
            }

            if mv.velocity.x <= 0.0 &&
                HumanoidMovement::collides_with_left_wall(mv, bb, &mut left_wall_x, terrain)
            {
                if mv.old_position.x - bb.aabb.half_size.x + bb.aabb.offset.x >= left_wall_x {
                    mv.position.x = left_wall_x + bb.aabb.half_size.x - bb.aabb.offset.x;
                    bb.pushes_left_wall = true;
                };
                mv.velocity.x = mv.velocity.x.max(0.0);
                mv.accel.x = mv.accel.x.max(0.0);
            } else {
                bb.pushes_left_wall = false;
            }

            if mv.velocity.x >= 0.0 &&
                HumanoidMovement::collides_with_right_wall(mv, bb, &mut right_wall_x, terrain)
            {
                if mv.old_position.x + bb.aabb.half_size.x + bb.aabb.offset.x <= right_wall_x {
                    mv.position.x = right_wall_x - bb.aabb.half_size.x - bb.aabb.offset.x;
                    bb.pushes_right_wall = true;
                }
                mv.velocity.x = mv.velocity.x.min(0.0);
                mv.accel.x = mv.accel.x.min(0.0);
            } else {
                bb.pushes_right_wall = false;
            }


            if mv.velocity.y >= 0.0 &&
                HumanoidMovement::has_ceiling(mv, bb, &mut ceiling_y, terrain)
            {
                mv.position.y = ceiling_y - bb.aabb.half_size.y - bb.aabb.offset.y - 1.0;
                mv.velocity.y = 0.0;
                bb.at_ceiling = true;
            } else {
                bb.at_ceiling = false;
            }
        });
    }
}

pub struct CollisionSystem;
impl<'a> System<'a> for CollisionSystem {
    type SystemData = (Entities<'a>,
     WriteStorage<'a, MovingObject>,
     ReadStorage<'a, HasAABB>,
     ReadStorage<'a, CollisionDetection>,
     Fetch<'a, LevelTerrain>);

    fn run(&mut self, data: Self::SystemData) {
        use physics::quad_tree::*;
        use rand;

        let (e, mut mv, bb, cd, t) = data;

        let terrain_rect = {
            let x = t.terrain.position.x;
            let y = t.terrain.position.y;
            let w = t.terrain.width as f64 * t.terrain.tile_size;
            let h = t.terrain.height as f64 * t.terrain.tile_size;
            Volume::new(x, y, w, h)
        };

        let mut qt = QuadTree::new(terrain_rect);

        {
            for (e, mv, bb, _) in (&*e, &mv, &bb, &cd).join() {
                let rect = (mv, bb).to_rect();
                qt.insert(e.clone(), rect);
            }
        }

        let qt = qt;

        (&*e, &mut mv, &bb, &cd).par_join().for_each(|(e, mv, bb, _)| {
            let vol = (&*mv, bb).to_rect();

            let iter = qt.retrieve(vol);

            for (ce, cv) in iter {
                if vol.intersects(&cv) && e != ce {
                    mv.velocity +=
                        Vector2::new(rand::random::<f64>() - 0.5, rand::random::<f64>() - 0.5)
                            .normalize() * 100.0;
                }
            }
        })
    }
}

pub struct PositionSystem;

impl<'a> System<'a> for PositionSystem {
    type SystemData = (ReadStorage<'a, MovingObject>, WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (mv, mut pos) = data;
        (&mv, &mut pos).par_join().for_each(|(mv, pos)| {
            pos.x = mv.position.x as f32;
            pos.y = mv.position.y as f32;
        });
    }
}
