use specs::*;
use std::iter;

pub struct HasAnimationSequence {
    pub sequence: AnimationSequence
}

impl Component for HasAnimationSequence {
    type Storage = HashMapStorage<HasAnimationSequence>;
}

// unsafe impl Send for AnimationSequence {}
// unsafe impl Sync for AnimationSequence {}

#[derive(Clone)]
pub enum Animation {
    Play { start: usize, end: usize },
    ReversePlay { start: usize, end: usize },
    Repeat {
        times: usize,
        animation: Box<Animation>,
    },
    Forever { animation: Box<Animation> },
    Pieces { pieces: Vec<Animation> },
}

#[derive(Clone)]
pub struct AnimationSequence {
    animation: Animation,
    current: Animation,
    leaf: Option<Box<AnimationSequence>>,
}

impl Iterator for AnimationSequence {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(frame) = next_frame(&mut self.leaf) {
            Some(frame)
        } else {
            match *&mut self.current {
                Animation::Play {
                    ref mut start,
                    ref end,
                } => {
                    if *start <= *end {
                        *start += 1;
                        Some(*start - 1)
                    } else {
                        None
                    }
                }
                Animation::ReversePlay {
                    ref mut start,
                    ref end,
                } => {
                    if *start >= *end {
                        *start -= 1;
                        Some(*start + 1)
                    } else {
                        None
                    }
                }
                Animation::Repeat {
                    ref mut times,
                    ref animation,
                } => {
                    if *times > 0 {
                        *times -= 1;
                        self.leaf = Some(Box::new(AnimationSequence::new(*animation.clone())));
                        next_frame(&mut self.leaf)
                    } else {
                        None
                    }
                }
                Animation::Forever { ref animation } => {
                    self.leaf = Some(Box::new(AnimationSequence::new(*animation.clone())));
                    next_frame(&mut self.leaf)
                }
                Animation::Pieces { ref mut pieces } => {
                    if pieces.len() > 0 {
                        self.leaf = Some(Box::new(AnimationSequence::new(pieces.remove(0))));
                        next_frame(&mut self.leaf)
                    } else {
                        None
                    }
                }
            }
        }
    }
}

fn next_frame(opt: &mut Option<Box<AnimationSequence>>) -> Option<usize> {
    let mut res = None;
    for mut iter in opt.iter_mut() {
        for ret in iter.next().iter() {
            res = Some(*ret)
        }
    }
    res
}


impl AnimationSequence {
    pub fn new(anim: Animation) -> AnimationSequence {
        AnimationSequence {
            animation: anim.clone(),
            current: anim,
            leaf: None,
        }
    }

    pub fn reset(&mut self) {
        self.current = self.animation.clone();
    }

    pub fn cycle(&mut self) -> usize {
        if let Some(fr) = self.next() {
            return fr;
        } else {
            self.reset();
            self.cycle()
        }
    }

    pub fn is_over(&mut self) -> bool {
        self.peekable().peek().is_none()
    }
}
