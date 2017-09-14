use level::*;
use std::time::Duration;

pub struct DeltaTime {
    pub time: Duration,
}

pub struct LevelTerrain {
    pub terrain: Terrain,
}

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
