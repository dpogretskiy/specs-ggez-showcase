use level::Terrain;
use physics::*;
use specs::*;
use std::cmp;
use util::*;

#[derive(Debug, Component)]
#[component(DenseVecStorage)]
pub struct MovingObject {
    pub old_position: Vector2,
    pub position: Vector2,

    pub old_accel: Vector2,
    pub accel: Vector2,

    pub old_velocity: Vector2,
    pub velocity: Vector2,
}

#[derive(Debug, Component)]
#[component(DenseVecStorage)]
pub struct HasAABB {
    pub aabb: AABB,

    pub pushed_right_wall: bool,
    pub pushes_right_wall: bool,

    pub pushed_left_wall: bool,
    pub pushes_left_wall: bool,

    pub was_on_ground: bool,
    pub on_ground: bool,

    pub was_on_platform: bool,
    pub on_platform: bool,

    pub was_at_ceiling: bool,
    pub at_ceiling: bool,

    pub cannot_go_left_frames: usize,
    pub cannot_go_right_frames: usize,

    pub frames_from_jump_start: usize,
}

impl MovingObject {
    pub fn new(position: Vector2) -> MovingObject {
        MovingObject {
            old_position: position.clone(),
            position: position,
            old_accel: Vector2::new(0.0, 0.0),
            accel: Vector2::new(0.0, 0.0),
            old_velocity: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
        }
    }
}

impl HasAABB {
    pub fn new(aabb: AABB) -> HasAABB {
        HasAABB {
            aabb: aabb,
            pushed_right_wall: false,
            pushed_left_wall: false,
            pushes_left_wall: false,
            pushes_right_wall: false,
            was_on_ground: false,
            on_ground: false,
            was_on_platform: false,
            on_platform: false,
            was_at_ceiling: false,
            at_ceiling: false,
            cannot_go_left_frames: 0,
            cannot_go_right_frames: 0,
            frames_from_jump_start: 0,
        }
    }
}


pub struct HumanoidMovement;
impl HumanoidMovement {
    pub fn has_ground(
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        ground_y: &mut f64,
        terrain: &Terrain,
    ) -> bool {
        let old_bottom_left =
            bb.aabb.sensor(&mv.old_position, Sensor::BottomLeft).right().down().ok();
        let new_bottom_left = bb.aabb.sensor(&mv.position, Sensor::BottomLeft).right().down().ok();
        let end_y = terrain.get_tile_y_at_point(new_bottom_left.y);
        let beg_y = cmp::max(terrain.get_tile_y_at_point(old_bottom_left.y) - 1, end_y);
        let dist = cmp::max((end_y - beg_y).abs(), 1);
        let mut tile_index_x;
        for tile_index_y in (end_y..beg_y + 1).rev() {
            let bottom_left = lerp(
                &new_bottom_left,
                &old_bottom_left,
                (end_y - tile_index_y).abs() as f64 / dist as f64,
            );

            let bottom_right =
                Vector2::new(bottom_left.x + bb.aabb.half_size.x * 2.0, bottom_left.y)
                    .left()
                    .left();
            let mut checked_tile = bottom_left.clone();
            'inner: loop {
                checked_tile.x = checked_tile.x.min(bottom_right.x);
                tile_index_x = terrain.get_tile_x_at_point(checked_tile.x);
                *ground_y = tile_index_y as f64 * terrain.tile_size + terrain.tile_size / 2.0 +
                    terrain.position.y;
                if terrain.is_obstacle(tile_index_x, tile_index_y) {
                    bb.on_platform = false;
                    return true;
                } else if terrain.is_one_way_platform(tile_index_x, tile_index_y) &&
                           (checked_tile.y - *ground_y).abs() <=
                               (HumanoidMovement::PLATFORM_THRESHOLD + mv.old_position.y -
                                    mv.position.y)
                {
                    bb.on_platform = true;
                };

                if checked_tile.x >= bottom_right.x {
                    if bb.on_platform {
                        return true;
                    }
                    break 'inner;
                }
                checked_tile.x += terrain.tile_size;
            }
        }

        false
    }

    pub fn has_ceiling(
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        ceiling_y: &mut f64,
        terrain: &Terrain,
    ) -> bool {
        *ceiling_y = 0.0;
        let old_top_right = bb.aabb.sensor(&mv.old_position, Sensor::TopRight).left().up().ok();
        let new_top_right = bb.aabb.sensor(&mv.position, Sensor::TopRight).left().up().ok();
        let end_y = terrain.get_tile_y_at_point(new_top_right.y);
        let beg_y = cmp::min(terrain.get_tile_y_at_point(old_top_right.y) + 1, end_y);
        let dist = cmp::max((end_y - beg_y).abs(), 1);
        let mut tile_index_x;
        for tile_index_y in beg_y..end_y + 1 {
            let top_right = lerp(
                &new_top_right,
                &old_top_right,
                ((end_y - tile_index_y).abs() as f64 / dist as f64),
            );
            let top_left = Vector2::new(top_right.x - bb.aabb.half_size.x * 2.0, top_right.y)
                .right()
                .right();
            let mut checked_tile = top_left.clone();
            loop {
                checked_tile.x = checked_tile.x.min(top_right.x);
                tile_index_x = terrain.get_tile_x_at_point(checked_tile.x);
                if terrain.is_obstacle(tile_index_x, tile_index_y) {
                    *ceiling_y = tile_index_y as f64 * terrain.tile_size - terrain.tile_size / 2.0 +
                        terrain.position.y;
                    return true;
                }
                if checked_tile.x >= top_right.x {
                    break;
                }
                checked_tile.x += terrain.tile_size;
            }
        }
        false
    }

    pub fn collides_with_left_wall(
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        wall_x: &mut f64,
        terrain: &Terrain,
    ) -> bool {
        *wall_x = 0.0;
        let old_bottom_left = bb.aabb.sensor(&mv.old_position, Sensor::BottomLeft).left().ok();
        let new_bottom_left = bb.aabb.sensor(&mv.position, Sensor::BottomLeft).left().ok();
        let mut tile_index_y;
        let end_x = terrain.get_tile_x_at_point(new_bottom_left.x);
        let beg_x = cmp::max(terrain.get_tile_x_at_point(old_bottom_left.x) - 1, end_x);
        let dist = cmp::max((end_x - beg_x).abs(), 1);
        for tile_index_x in (end_x..beg_x + 1).rev() {
            let bottom_left = lerp(
                &new_bottom_left,
                &old_bottom_left,
                (end_x - tile_index_x).abs() as f64 / dist as f64,
            );
            let top_left = bottom_left + Vector2::new(0.0, bb.aabb.half_size.y * 2.0);
            let mut checked_tile = bottom_left;
            loop {
                checked_tile.y = checked_tile.y.min(top_left.y);
                tile_index_y = terrain.get_tile_y_at_point(checked_tile.y);
                if terrain.is_obstacle(tile_index_x, tile_index_y) {
                    *wall_x = tile_index_x as f64 * terrain.tile_size + terrain.tile_size / 2.0 +
                        terrain.position.x;
                    return true;
                }
                if checked_tile.y >= top_left.y {
                    break;
                }
                checked_tile.y += terrain.tile_size;
            }
        }
        false
    }

    pub fn collides_with_right_wall(
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        wall_x: &mut f64,
        terrain: &Terrain,
    ) -> bool {
        *wall_x = 0.0;
        let old_bottom_right =
            bb.aabb.sensor(&mv.old_position, Sensor::BottomRight).right().right().ok();
        let new_bottom_right =
            bb.aabb.sensor(&mv.position, Sensor::BottomRight).right().right().ok();
        let end_x = terrain.get_tile_x_at_point(new_bottom_right.x);
        let beg_x = cmp::min(terrain.get_tile_x_at_point(old_bottom_right.x) + 1, end_x);
        let dist = cmp::max((end_x - beg_x).abs(), 1);
        let mut tile_index_y;
        for tile_index_x in beg_x..end_x + 1 {
            let bottom_right = lerp(
                &new_bottom_right,
                &old_bottom_right,
                (end_x - tile_index_x).abs() as f64 / dist as f64,
            );
            let top_right = bottom_right + Vector2::new(0.0, bb.aabb.half_size.y * 2.0);
            let mut checked_tile = bottom_right;
            loop {
                checked_tile.y = checked_tile.y.min(top_right.y);
                tile_index_y = terrain.get_tile_y_at_point(checked_tile.y);
                if terrain.is_obstacle(tile_index_x, tile_index_y) {
                    *wall_x = tile_index_x as f64 * terrain.tile_size - terrain.tile_size / 2.0 +
                        terrain.position.x;
                    return true;
                }
                if checked_tile.y >= top_right.y {
                    break;
                }
                checked_tile.y += terrain.tile_size;
            }
        }
        false
    }

    pub const PLATFORM_THRESHOLD: f64 = 2.0;
}


#[derive(Debug, Component)]
#[component(VecStorage)]
pub struct CollisionDetection {
    pub group: usize,
}
