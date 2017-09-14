use std::collections::HashMap;

use super::LoadedAssets;
use marker::*;
use marker::geom::Rect;

use ggez::graphics::Image;
use ggez::graphics::spritebatch::*;

use rand;

#[derive(Debug)]
struct GroundIndex {
    square: HashMap<Square, Vec<Rect>>,
}
#[derive(Debug)]
struct PlatformIndex {
    horizontal: HashMap<Horizontal, Vec<Rect>>,
}
#[derive(Debug)]
struct ObjectIndex {
    ground: Vec<Rect>,
    surface: Vec<Rect>,
}

#[derive(Debug)]
pub struct LevelAssetIndex {
    ground: GroundIndex,
    platforms: PlatformIndex,
    objects: ObjectIndex,
}

impl LevelAssetIndex {
    pub fn build(loaded: &LoadedAssets) -> LevelAssetIndex {
        let mut ground_sqr: HashMap<Square, Vec<Rect>> = HashMap::new();
        let mut platform_hor: HashMap<Horizontal, Vec<Rect>> = HashMap::with_capacity(3);
        let mut ground_obj = vec![];
        let mut surface_obj = vec![];

        for gd in loaded.ground.data.iter() {
            match &gd.markers {
                &SpriteType::Ground { square: ref sqr } => for s in sqr.iter() {
                    let mut p = true;
                    {
                        let entry = ground_sqr.get_mut(s);
                        if let Some(e) = entry {
                            e.push(gd.on_screen_frame.clone());
                            p = false;
                        };
                    }
                    if p {
                        ground_sqr.insert(s.clone(), vec![gd.on_screen_frame.clone()]);
                    };
                },
                &SpriteType::Platform {
                    horizontal: ref hor,
                } => for h in hor.iter() {
                    platform_hor.entry(h.clone()).or_insert({ vec![gd.on_screen_frame.clone()] });
                },
                &SpriteType::Object => ground_obj.push(gd.on_screen_frame.clone()),
            }
        }

        for od in loaded.objects.data.iter() {
            match od.markers {
                SpriteType::Object => surface_obj.push(od.on_screen_frame.clone()),
                _ => (),
            }
        }

        let index = LevelAssetIndex {
            ground: GroundIndex { square: ground_sqr },
            objects: ObjectIndex {
                ground: ground_obj,
                surface: surface_obj,
            },
            platforms: PlatformIndex {
                horizontal: platform_hor,
            },
        };

        index
    }

    pub fn find_ground(&self, sqr: Square) -> Option<Rect> {
        self.ground.square.get(&sqr).and_then(random_from)
    }
    pub fn find_platform(&self, hor: Horizontal) -> Option<Rect> {
        self.platforms.horizontal.get(&hor).and_then(random_from)
    }

    pub fn find_object(&self, surface: bool) -> Option<Rect> {
        let r = if surface {
            &self.objects.surface
        } else {
            &self.objects.ground
        };
        random_from(&r)
    }
}


fn random_from<T: Clone>(from: &Vec<T>) -> Option<T> {
    if from.len() > 0 {
        let ix = rand::random::<usize>() % from.len();
        Some(from[ix].clone())
    } else {
        None
    }
}
