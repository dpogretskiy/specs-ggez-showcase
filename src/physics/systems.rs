use components::Position;
use physics::components::*;
use rayon::iter::ParallelIterator;
use resources::*;
use specs::*;
use util::*;

pub struct MovingSystem;
impl<'a> System<'a> for MovingSystem {
    type SystemData = (WriteStorage<'a, MovingObject>, Fetch<'a, DeltaTime>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut objects, time) = data;
        let delta = seconds(&time.time);

        (&mut objects).par_join().for_each(|o| {
            o.old_position = o.position;
            o.old_velocity = o.velocity;
            o.old_accel = o.accel;
            o.velocity += o.accel * delta;
            o.position += o.velocity * delta;
        });
    }
}

pub struct HasAABBSystem;
impl<'a> System<'a> for HasAABBSystem {
    type SystemData = (WriteStorage<'a, HasAABB>,
     WriteStorage<'a, MovingObject>,
     Fetch<'a, LevelTerrain>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut has_aabb, mut mv, level) = data;
        let terrain = &level.terrain;

        (&mut has_aabb, &mut mv).par_join().for_each(|(bb, mv)| {
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

            bb.aabb.center = mv.position + bb.aabb.offset;
        });
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
