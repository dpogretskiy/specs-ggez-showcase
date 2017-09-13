
use super::*;

use sprite::Loader;

use ggez::Context;
use ggez::graphics::Image;


pub struct LoadedAssets {
    pub ground: MarkedTiles,
    pub objects: MarkedTiles,
    pub background: Image,
}

impl LoadedAssets {
    pub fn load_assets<'a>(ctx: &mut Context, tpe: LevelType) -> GameResult<LoadedAssets> {
        let (g, o, bg) = match tpe {
            LevelType::Graveyard => {
                let g = Loader::load_sprite_sheet(ctx, "/level/graveyard/level_ground")?;
                let o = Loader::load_sprite_sheet(ctx, "/level/graveyard/level_objects")?;
                let bg = Image::new(ctx, "/level/graveyard/background.png")?;
                (g, o, bg)
            }
        };
        Ok(LoadedAssets {
            ground: g,
            objects: o,
            background: bg,
        })
    }
}
