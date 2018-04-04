use ggez;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect { x, y, w, h }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

impl From<ggez::graphics::Rect> for Rect {
    fn from(r: ggez::graphics::Rect) -> Rect {
        Rect {
            x: r.x,
            y: r.y,
            w: r.w,
            h: r.h,
        }
    }
}

impl From<Rect> for ggez::graphics::Rect {
    fn from(r: Rect) -> ggez::graphics::Rect {
        ggez::graphics::Rect {
            x: r.x,
            y: r.y,
            w: r.w,
            h: r.h,
        }
    }
}
