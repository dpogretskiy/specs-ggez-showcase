
use na;
use std::time::Duration;

pub type Vector2 = na::Vector2<f64>;

pub fn seconds(dur: &Duration) -> f64 {
    dur.as_secs() as f64 + (dur.subsec_nanos() as f64 / 1000000000.0)
}

pub fn lerp(v1: &Vector2, v2: &Vector2, by: f64) -> Vector2 {
    v1 * (1.0 - by) + v2 * by
}

pub fn round_vector(mut v: Vector2) -> Vector2 {
    v.x = v.x.round();
    v.y = v.y.round();
    v
}
