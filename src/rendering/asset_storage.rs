use ggez::graphics::Image;
use ggez::graphics::spritebatch::SpriteBatch;
use sprite::animation::Animation;
use std::collections::HashMap;

pub struct AssetStorage {
    pub images: HashMap<&'static str, Image>,
    pub animations: HashMap<&'static str, Animation>,
    pub batches: HashMap<&'static str, SpriteBatch>,
}

impl AssetStorage {
    pub fn empty() -> AssetStorage {
        let m1 = HashMap::new();
        let m2 = HashMap::new();
        let m3 = HashMap::new();

        AssetStorage {
            images: m1,
            animations: m2,
            batches: m3,
        }
    }
}
