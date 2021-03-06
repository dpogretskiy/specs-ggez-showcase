use level::*;

#[derive(Clone)]
pub struct DeltaTime {
    pub delta: f64,
}

pub struct LevelTerrain {
    pub terrain: Terrain,
}

#[derive(Clone)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub slide: bool,
    pub jump: bool,
    pub attack: bool,
}

impl PlayerInput {
    pub fn new() -> PlayerInput {
        PlayerInput {
            up: false,
            down: false,
            left: false,
            right: false,
            slide: false,
            jump: false,
            attack: false,
        }
    }

    pub fn reset_actions(&mut self) {
        self.attack = false;
        self.slide = false;
        self.jump = false;
    }
}

#[derive(Clone)]
pub struct MousePointer(pub f64, pub f64);
