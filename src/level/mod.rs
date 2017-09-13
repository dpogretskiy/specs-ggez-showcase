mod index;
mod loaded;
mod terrain;

use ggez::graphics::Image;
use ggez::{Context, GameResult};
use std::rc::Rc;
use super::Vector2;

use sprite::MarkedTiles;
use marker::geom::*;

use ggez::graphics::DrawParam;
use ggez::graphics;
use ggez::graphics::spritebatch::*;
use ggez::graphics::Drawable;

use self::index::*;
use marker::{Horizontal, Square};

pub use self::loaded::*;
pub use self::terrain::*;


use self::index::LevelAssetIndex;

pub enum LevelType {
    Graveyard,
}

pub struct Level {
    pub terrain_data: Vec<Vec<usize>>,
    pub index: LevelAssetIndex,
    pub assets: LoadedAssets,
}

impl Level {
    pub fn load(ctx: &mut Context, lt: LevelType) -> GameResult<Level> {
        let assets = LoadedAssets::load_assets(ctx, lt)?;
        let terrain_data = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        let index = LevelAssetIndex::build(&assets);

        Ok(Level {
            index,
            terrain_data,
            assets,
        })
    }
}

pub struct RenderableLevel {
    pub background: Image,
    pub sprites: Vec<SpriteBatch>,
    pub terrain: Terrain,
}

impl RenderableLevel {
    pub fn build(level: Level) -> RenderableLevel {
        let index: LevelAssetIndex;
        let terrain_data: Vec<Vec<usize>>;
        let assets: LoadedAssets;

        {
            index = level.index;
            terrain_data = level.terrain_data;
            assets = level.assets;
        }

        let mut g_batch = SpriteBatch::new(assets.ground.image);
        let mut o_batch = SpriteBatch::new(assets.objects.image);
        let bg = assets.background;


        let mut terrain_vec: Vec<Vec<TileType>> = vec![];

        let height = terrain_data.len();
        let pixel_height = height * 128;
        let width = terrain_data[0].len();

        for v_vec in terrain_data.iter() {
            let mut h_vec = vec![];

            for tile in v_vec.iter() {
                match tile {
                    &0 => h_vec.push(TileType::Empty),
                    &1 => h_vec.push(TileType::Block),
                    &2 => h_vec.push(TileType::OneWay),
                    _ => h_vec.push(TileType::Empty),
                }
            }


            h_vec.shrink_to_fit();
            terrain_vec.push(h_vec);
        }

        {
            let is_left_wall = |h| h == 0;
            let is_right_wall = |h| h == width - 1;
            let is_floor = |v| v == height - 1;
            let is_roof = |v| v == 0;
            let is_corner =
                |h, v| (is_left_wall(h) || is_right_wall(h)) && (is_floor(v) || is_roof(v));

            let lookup = |(h, v, t): (usize, usize, &Vec<Vec<usize>>)| -> Option<Rect> {
                let it = t[v][h];
                if it == 0 {
                    None
                } else {
                    let itself = t[v][h];
                    let above = t[v - 1][h];
                    let below = t[v + 1][h];
                    let on_left = t[v][h - 1];
                    let on_right = t[v][h + 1];
                    let above_right = t[v - 1][h + 1];
                    let above_left = t[v - 1][h - 1];
                    let bottom_left = t[v + 1][h - 1];
                    let bottom_right = t[v + 1][h + 1];

                    let mat = (
                        (above_left, above, above_right),
                        (on_left, itself, on_right),
                        (bottom_left, below, bottom_right),
                    );

                    if it == 1 {
                        match mat {
                            ((l, 1, r), (1, 1, 1), (_, 1, _)) => if l != 0 && r != 0 {
                                index.find_ground(Square::MM)
                            } else if r != 0 {
                                index.find_ground(Square::IBR)
                            } else {
                                index.find_ground(Square::IBL)
                            },
                            ((l, 0, r), (1, 1, 1), (_, 1, _)) => if l != 0 {
                                index.find_ground(Square::ILT)
                            } else if r != 0 {
                                index.find_ground(Square::IRT)
                            } else {
                                index.find_ground(Square::MT)
                            },
                            ((_, a, _), (l, 1, r), (_, b, _)) => if a == 0 {
                                if l == 0 {
                                    index.find_ground(Square::LT)
                                } else if r == 0 {
                                    index.find_ground(Square::RT)
                                } else {
                                    index.find_ground(Square::MT)
                                }
                            } else if b == 0 {
                                if l == 0 {
                                    index.find_ground(Square::LB)
                                } else if r == 0 {
                                    index.find_ground(Square::RB)
                                } else {
                                    index.find_ground(Square::MB)
                                }
                            } else {
                                if l == 0 {
                                    index.find_ground(Square::LM)
                                } else if r == 0 {
                                    index.find_ground(Square::RM)
                                } else {
                                    index.find_ground(Square::MM)
                                }
                            },
                            _ => None,
                        }
                    } else if it == 2 {
                        if on_left != 0 && on_right != 0 {
                            index.find_platform(Horizontal::Center)
                        } else if on_left == 0 {
                            index.find_platform(Horizontal::Left)
                        } else if on_right == 0 {
                            index.find_platform(Horizontal::Right)
                        } else {
                            index.find_platform(Horizontal::Center)
                        }
                    } else {
                        None
                    }
                }
            };

            let t: &Vec<Vec<usize>> = &terrain_data;

            for h in 0..width {
                for v in 0..height {
                    let rect: Option<Rect> = if is_left_wall(h) && is_floor(v) {
                        index.find_ground(Square::IBL)
                    } else if is_floor(v) && h == 1 {
                        index.find_ground(Square::ILT)
                    } else if is_floor(v) && h == (width - 2) {
                        index.find_ground(Square::IRT)
                    } else if is_right_wall(h) && is_floor(v) {
                        index.find_ground(Square::IBR)
                    } else if is_corner(h, v) {
                        index.find_ground(Square::MM)
                    } else if is_left_wall(h) {
                        if t[v][h + 1] != 0 {
                            index.find_ground(Square::MM)
                        } else {
                            index.find_ground(Square::RM)
                        }
                    } else if is_right_wall(h) {
                        if t[v][h - 1] != 0 {
                            index.find_ground(Square::MM)
                        } else {
                            index.find_ground(Square::LM)
                        }
                    } else if is_floor(v) {
                        if t[v - 1][h] != 0 {
                            index.find_ground(Square::MM)
                        } else {
                            index.find_ground(Square::MT)
                        }
                    } else if is_roof(v) {
                        index.find_ground(Square::MB)
                    } else {
                        lookup((h, v, t))
                    };

                    if let Some(rect) = rect {
                        let dp = DrawParam {
                            src: graphics::Rect::from(rect),
                            dest: graphics::Point::new(
                                (h * 128) as f32,
                                (pixel_height - v * 128) as f32,
                            ),
                            scale: graphics::Point::new(1.0, 1.0),
                            ..Default::default()
                        };

                        g_batch.add(dp);
                    };
                }
            }
        };

        terrain_vec.reverse();
        RenderableLevel {
            background: bg,
            sprites: vec![g_batch, o_batch],
            terrain: Terrain {
                terrain: terrain_vec,
                position: Vector2::new(0.0, 128.0),
                width: width,
                height: height,
                tile_size: 128.0,
            },
        }
    }
}

impl Drawable for RenderableLevel {
    fn draw_ex(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        for spr in self.sprites.iter() {
            spr.draw_ex(ctx, param)?;
        }
        Ok(())
    }
}