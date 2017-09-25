use util::*;

#[derive(Debug, Clone)]
pub struct AABB {
    pub half_size: Vector2,
    pub scale: Vector2,
    pub offset: Vector2,
}

impl AABB {
    pub fn new_full(full_size: Vector2, scale: Vector2) -> AABB {
        let half_size = full_size / 2.0;
        let offset_y = (-half_size.y * (1.0 - scale.y)).round();

        let scaled_hs = Vector2::new(
            (half_size.x * scale.x).round(),
            (half_size.y * scale.y).round(),
        );

        AABB {
            half_size: scaled_hs,
            scale,
            offset: Vector2::new(0.0, offset_y),
        }
    }

    // pub fn overlaps(&self, other: &AABB) -> bool {
    //     !(self.center.x - other.center.x > self.half_size().x + other.half_size().x) &&
    //         !(self.center.y - other.center.y > self.half_size().y + other.half_size().y)
    // }

    pub fn sensor(&self, at: &Vector2, which: Sensor) -> SensorBuilder {
        let vector = match which {
            Sensor::BottomLeft => at - self.half_size + self.offset,
            Sensor::BottomRight => {
                at + Vector2::new(self.half_size.x, -self.half_size.y) + self.offset
            }
            Sensor::TopRight => at + self.half_size + self.offset,
        };

        SensorBuilder {
            vector: round_vector(vector),
            hor: 0,
            ver: 0,
        }
    }
}

pub enum Sensor {
    BottomLeft,
    TopRight,
    BottomRight,
}

pub struct SensorBuilder {
    vector: Vector2,
    hor: usize,
    ver: usize,
}

impl SensorBuilder {
    pub fn ok(self) -> Vector2 {
        round_vector(self.vector)
    }
}

pub trait Disposition {
    #[inline(always)]
    fn up(self) -> Self;
    #[inline(always)]
    fn down(self) -> Self;
    #[inline(always)]
    fn left(self) -> Self;
    #[inline(always)]
    fn right(self) -> Self;
}

const BY: f64 = 1.0;

impl Disposition for SensorBuilder {
    fn up(mut self) -> SensorBuilder {
        self.vector.y += BY;
        self.ver += 1;
        self
    }
    fn down(mut self) -> SensorBuilder {
        self.vector.y -= BY;
        self.ver += 1;
        self
    }
    fn left(mut self) -> SensorBuilder {
        self.vector.x -= BY;
        self.hor += 1;
        self
    }
    fn right(mut self) -> SensorBuilder {
        self.vector.x += BY;
        self.hor += 1;
        self
    }
}

impl Disposition for Vector2 {
    fn up(mut self) -> Vector2 {
        self.y += BY;
        self
    }
    fn down(mut self) -> Vector2 {
        self.y -= BY;
        self
    }
    fn left(mut self) -> Vector2 {
        self.x -= BY;
        self
    }
    fn right(mut self) -> Vector2 {
        self.x += BY;
        self
    }
}
