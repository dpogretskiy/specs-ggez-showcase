use super::MarkedTiles;

use ggez::graphics::Rect;
use ggez::graphics::spritebatch::SpriteBatch;

pub struct Animation {
    pub batch: SpriteBatch,
    pub frames: Vec<Rect>,
    pub length: usize,
}

impl Animation {
    pub fn new(mt: MarkedTiles) -> Animation {
        let length = mt.data.len();

        Animation {
            batch: SpriteBatch::new(mt.image),
            frames: mt.data.iter().map(|f| Rect::from(f.on_screen_frame.clone())).collect(),
            length,
        }
    }
}
