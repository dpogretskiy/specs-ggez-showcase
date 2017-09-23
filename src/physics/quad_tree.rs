use specs::Entity;

use physics::components::*;
use std::cell::RefCell;

pub trait Positioned {
    fn to_rect(&self) -> Volume;
}

pub struct QuadTree {
    level: usize,
    bounds: Volume,
    objects: Vec<(Entity, Volume)>,
    nodes: Option<Box<[QuadTree; 4]>>,
}

impl QuadTree {
    pub fn new(bounds: Volume) -> QuadTree {
        QuadTree::create(0, bounds)
    }

    fn create(level: usize, bounds: Volume) -> QuadTree {
        QuadTree {
            level,
            bounds,
            objects: vec![],
            nodes: None,
        }
    }

    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }

    fn split(&mut self) {
        let sub_width = self.bounds.w / 2.0;
        let sub_height = self.bounds.h / 2.0;

        let x = self.bounds.x;
        let y = self.bounds.y;

        let level = self.level + 1;

        self.nodes = Some(Box::new(
            [
                QuadTree::create(
                    level,
                    Volume::new(x + sub_width, y, sub_width, sub_height),
                ),
                QuadTree::create(
                    level,
                    Volume::new(x, y, sub_width, sub_height),
                ),
                QuadTree::create(
                    level,
                    Volume::new(x, y + sub_height, sub_width, sub_height),
                ),
                QuadTree::create(
                    level,
                    Volume::new(
                        x + sub_width,
                        y + sub_height,
                        sub_width,
                        sub_height,
                    ),
                ),
            ],
        ));
        self.nodes = Some(Box::new(
            [
                QuadTree::create(
                    level,
                    Volume::new(x + sub_width, y, sub_width, sub_height),
                ),
                QuadTree::create(
                    level,
                    Volume::new(x, y, sub_width, sub_height),
                ),
                QuadTree::create(
                    level,
                    Volume::new(x, y + sub_height, sub_width, sub_height),
                ),
                QuadTree::create(
                    level,
                    Volume::new(
                        x + sub_width,
                        y + sub_height,
                        sub_width,
                        sub_height,
                    ),
                ),
            ],
        ));
    }

    pub fn insert(&mut self, entity: Entity, rect: Volume) {
        if let Some(ref mut nodes) = self.nodes {
            let index = get_index(&self.bounds, &rect);
            if index != -1 {
                (*nodes)[index as usize].insert(entity, rect.clone());
                return;
            }
        }

        self.objects.push((entity, rect));

        if self.objects.len() > MAX_OBJECTS && self.level < MAX_LEVELS {
            if self.nodes.is_none() {
                self.split();
            }

            for o in self.objects.drain(..) {
                let ix = get_index(&self.bounds, &o.1);

                if ix != -1 {
                    if let Some(ref mut nodes) = self.nodes {
                        (*nodes)[ix as usize].insert(entity, rect);
                    }
                }
            }
        }
    }

    fn retrieve_rec(&self, ret: &mut Vec<(Entity, Volume)>, rect: Volume) {
        let ix = get_index(&self.bounds, &rect);
        if ix != -1 {
            if let Some(ref nodes) = self.nodes {
                (*nodes)[ix as usize].retrieve_rec(ret, rect);
            }
        }

        let owned_iter =
            self.objects.iter().map(|&(e, v)| (e.clone(), v.clone())).collect::<Vec<_>>();

        ret.extend(owned_iter);
    }

    pub fn retrieve(&self, rect: Volume) -> Vec<(Entity, Volume)> {
        let mut ret = vec![];
        self.retrieve_rec(&mut ret, rect);
        ret
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Volume {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Volume {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Volume {
        Volume { x, y, w, h }
    }

    pub fn intersects(&self, other: &Volume) -> bool {
        !(self.x + self.w < other.x || other.x + other.w < self.x || self.y + self.h < other.y ||
              other.y + other.h < self.y)
    }
}

impl<'a> Positioned for (&'a MovingObject, &'a HasAABB) {
    fn to_rect(&self) -> Volume {
        let &(mv, bb) = self;

        let xy = mv.position - bb.aabb.half_size + bb.aabb.offset;
        let wh = bb.aabb.half_size * 2.0;
        Volume::new(xy.x, xy.y, wh.x, wh.y)
    }
}

fn get_index(bounds: &Volume, rect: &Volume) -> isize {
    let vertical_midpoint = bounds.x + bounds.w / 2.0;
    let horizontal_midpoint = bounds.y + bounds.h / 2.0;

    let top_quadrant = rect.y < horizontal_midpoint && rect.y + rect.h < horizontal_midpoint;
    let bottom_quadrant = rect.y > horizontal_midpoint;

    let mut index = -1;

    if rect.x < vertical_midpoint && rect.x + rect.w < vertical_midpoint {
        if top_quadrant {
            index = 1;
        } else if bottom_quadrant {
            index = 2;
        }
    } else if rect.x > vertical_midpoint {
        if top_quadrant {
            index = 0;
        } else if bottom_quadrant {
            index = 3;
        }
    }
    index
}

const MAX_OBJECTS: usize = 10;
const MAX_LEVELS: usize = 5;
