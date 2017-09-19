pub mod components;
pub mod systems;
pub mod animation_defs;
pub mod state_machine;
pub mod state;
mod consts;

use asset_storage::*;
use ggez::{Context, GameResult};
use sprite::Loader;
use sprite::animation::Animation;

pub struct PlayerLoader {}
impl PlayerLoader {
    pub fn load_assets(ctx: &mut Context, asset_storage: &mut AssetStorage) -> GameResult<()> {
        let idle = Loader::load_sprite_sheet(ctx, "/idle")?;
        let attacking = Loader::load_sprite_sheet(ctx, "/attack")?;
        let jumping = Loader::load_sprite_sheet(ctx, "/jump")?;
        let running = Loader::load_sprite_sheet(ctx, "/run")?;
        let sliding = Loader::load_sprite_sheet(ctx, "/slide")?;

        asset_storage.animations.extend(vec![
            (P_IDLE, Animation::new(idle)),
            (P_ATTACK, Animation::new(attacking)),
            (P_JUMP, Animation::new(jumping)),
            (P_RUN, Animation::new(running)),
            (P_SLIDE, Animation::new(sliding)),
        ]);
        Ok(())
    }
}

const P_IDLE: &str = "player-idle";
const P_ATTACK: &str = "player-attack";
const P_JUMP: &str = "player-jump";
const P_RUN: &str = "player-run";
const P_SLIDE: &str = "player-slide";
