use ggez::{Context, GameError, GameResult};
use ggez::graphics::Image;
use marker::*;
use serde_json;


pub mod animation;

pub struct Loader;

impl Loader {
    pub fn load_sprite_sheet(ctx: &mut Context, name: &str) -> GameResult<MarkedTiles> {
        let data_file = ctx.filesystem.open(format!("{}.json", name)).or_else(|_| {
            ctx.filesystem.open(format!("{}-marked.json", name))
        })?;

        let data: Vec<SpriteData> = serde_json::from_reader(data_file).map_err(|_| {
            GameError::ResourceLoadError(format!("Data not found: {}", name))
        })?;
        let image = Image::new(ctx, format!("{}.png", name))?;

        Ok(MarkedTiles { data, image: image })
    }
}

pub struct MarkedTiles {
    pub data: Vec<SpriteData>,
    pub image: Image,
}
