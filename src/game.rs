use ggez::*;
use specs::*;

pub struct Game {
    world: World,
    level: Level,
}

pub struct Level;


impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let world = World::new();
        let level = Level;


        

    }
}