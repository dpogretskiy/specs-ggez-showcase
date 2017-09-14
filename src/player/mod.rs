mod components;
mod systems;
mod consts;

use asset_storage::*;
use components::*;
use ggez::{Context, GameResult};
use sprite::Loader;
use sprite::animation::Animation;
use systems::*;

pub struct PlayerLoader {}
impl PlayerLoader {
    pub fn load_assets(ctx: &mut Context, asset_storage: &mut AssetStorage) -> GameResult<()> {
        let idle = Loader::load_sprite_sheet(ctx, "/idle")?;
        let attacking = Loader::load_sprite_sheet(ctx, "/attack")?;
        let jumping = Loader::load_sprite_sheet(ctx, "/jump")?;
        let running = Loader::load_sprite_sheet(ctx, "/run")?;
        let sliding = Loader::load_sprite_sheet(ctx, "/slide")?;

        asset_storage.animations.extend(vec![
            ("player-idle", Animation::new(idle)),
            ("player-attack", Animation::new(attacking)),
            ("player-jump", Animation::new(jumping)),
            ("player-run", Animation::new(running)),
            ("player-slide", Animation::new(sliding)),
        ]);
        Ok(())
    }
}
