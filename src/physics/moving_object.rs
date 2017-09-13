use super::Vector2;

use level::Terrain;
use std::time::Duration;
use std::cmp;

#[derive(Debug, Clone, Component)]
#[component(HashMapStorage)]
pub struct MovingObject {
    pub old_position: Vector2,
    pub position: Vector2,

    pub old_accel: Vector2,
    pub accel: Vector2,

    pub old_velocity: Vector2,
    pub velocity: Vector2,

}

#[derive(Debug, Clone, Component)]
#[component(HashMapStorage)]
pub struct HasAABB {
    pub aabb: AABB,

    pub pushed_right_wall: bool,
    pub pushes_right_wall: bool,

    pub pushed_left_wall: bool,
    pub pushes_left_wall: bool,

    pub was_on_ground: bool,
    pub on_ground: bool,

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
            aabb: aabb,
            pushed_right_wall: false,
            pushed_left_wall: false,
            pushes_left_wall: false,
            pushes_right_wall: false,
            was_on_ground: false,
            on_ground: false,
            on_platform: false,
            was_at_ceiling: false,
            at_ceiling: false,
            cannot_go_left_frames: 0,
            cannot_go_right_frames: 0,
            frames_from_jump_start: 0,
    }
}

struct MovingSystem