pub mod geom;

use super::sprite::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub enum Square {
    LT,
    MT,
    RT,
    LM,
    MM,
    RM,
    LB,
    MB,
    RB,
    IBL,
    ILT,

    IBR,
    IRT,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub enum Horizontal {
    Left,
    Right,
    Center,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub enum SpriteType {
    Object,
    Platform { horizontal: Vec<Horizontal> },
    Ground { square: Vec<Square> },
}

impl SpriteType {
    pub fn empty_ground() -> SpriteType {
        SpriteType::Ground { square: vec![] }
    }

    pub fn empty_platform() -> SpriteType {
        SpriteType::Platform { horizontal: vec![] }
    }
}

// impl Serialize for SpriteType {}

// impl Deserialize for SpriteType {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpriteData {
    pub on_screen_frame: geom::Rect,
    pub frame: geom::Rect,
    pub markers: SpriteType,
    pub name: String,
    pub index: usize,
}
