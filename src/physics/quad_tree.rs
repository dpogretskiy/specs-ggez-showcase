use super::*;

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::iter;

use physics::components::*;

pub trait Positioned {
    fn to_rect(&self) -> Rect;
}

pub struct QuadTree<'a, T: 'a> {
    level: usize,
    bounds: Rect,
    objects: Vec<&'a T>,
    nodes: Option<RefCell<Box<[QuadTree<'a, T>; 4]>>>,
}

impl<'a, T> QuadTree<'a, T>
where
    T: Positioned, {
    pub fn new(bounds: Rect) -> QuadTree<'a, T> {
        QuadTree::create(0, bounds)
    }

    fn create(level: usize, bounds: Rect) -> QuadTree<'a, T> {
        QuadTree {
            level,
            bounds,
            objects: vec![],
            nodes: None,
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    fn split(&mut self) {
        let sub_width = self.bounds.w / 2.0;
        let sub_height = self.bounds.h / 2.0;

        let x = self.bounds.x;
        let y = self.bounds.y;

        let level = self.level + 1;

        self.nodes = Some(RefCell::new(Box::new(
            [
                QuadTree::create(
                    level,
                    Rect::new(x + sub_width, y, sub_width, sub_height),
                ),
                QuadTree::create(
                    level,
                    Rect::new(x, y, sub_width, sub_height),
                ),
                QuadTree::create(
                    level,
                    Rect::new(x, y + sub_height, sub_width, sub_height),
                ),
                QuadTree::create(
                    level,
                    Rect::new(
                        x + sub_width,
                        y + sub_height,
                        sub_width,
                        sub_height,
                    ),
                ),
            ],
        )));
        self.nodes = Some(RefCell::new(Box::new(
            [
                QuadTree::create(
                    level,
                    Rect::new(x + sub_width, y, sub_width, sub_height),
                ),
                QuadTree::create(
                    level,
                    Rect::new(x, y, sub_width, sub_height),
                ),
                QuadTree::create(
                    level,
                    Rect::new(x, y + sub_height, sub_width, sub_height),
                ),
                QuadTree::create(
                    level,
                    Rect::new(
                        x + sub_width,
                        y + sub_height,
                        sub_width,
                        sub_height,
                    ),
                ),
            ],
        )));
    }

    pub fn insert(&mut self, object: &'a T) {
        if let Some(ref nodes) = self.nodes {
            let index = get_index(&self.bounds, &object.to_rect());
            if index != -1 {
                nodes.borrow_mut()[index as usize].insert(object);
                return;
            }
        }

        self.objects.push(object);

        if self.objects.len() > MAX_OBJECTS && self.level < MAX_LEVELS {
            if self.nodes.is_none() {
                self.split();
            }

            for o in self.objects.drain(..) {
                let ix = get_index(&self.bounds, &o.to_rect());

                if ix != -1 {
                    if let Some(ref nodes) = self.nodes {
                        nodes.borrow_mut()[ix as usize].insert(o);
                    }
                }
            }
        }
    }

    fn retreive_rec(&self, ret: &mut Vec<&'a T>, rect: Rect) {
        let ix = get_index(&self.bounds, &rect);
        if ix != -1 {
            if let Some(ref nodes) = self.nodes {
                nodes.borrow()[ix as usize].retreive_rec(ret, rect);
            }
        }

        ret.extend(self.objects.iter());
    }

    pub fn retrieve(&self, rect: Rect) -> Vec<&'a T> {
        let mut ret = vec![];
        self.retreive_rec(&mut ret, rect);
        ret
    }
}

#[derive(Debug)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Rect {
        Rect { x, y, w, h }
    }
}

impl<'a> Positioned for (&'a MovingObject, &'a HasAABB) {
    fn to_rect(&self) -> Rect {
        let &(mv, bb) = self;

        let xy = mv.position - bb.aabb.half_size + bb.aabb.offset;
        let wh = bb.aabb.half_size * 2.0;
        Rect::new(xy.x, xy.y, wh.x, wh.y)
    }
}

fn get_index(bounds: &Rect, rect: &Rect) -> isize {
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
