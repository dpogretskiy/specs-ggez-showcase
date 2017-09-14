use super::MarkedTiles;

use ggez::graphics::{Image, Rect};

pub struct Animation {
    pub image: Image,
    pub frames: Vec<Rect>,
    pub length: usize,
}

impl Animation {
    pub fn new(mt: MarkedTiles) -> Animation {
        let length = mt.data.len();

        Animation {
            image: mt.image,
            frames: mt.data.iter().map(|f| Rect::from(f.on_screen_frame.clone())).collect(),
            length,
        }
    }
}
