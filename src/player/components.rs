use player::state_machine::StateMachine;
use specs::*;

#[derive(Debug, Component, Default)]
#[component(NullStorage)]
pub struct Controlled;

#[derive(Component)]
#[component(DenseVecStorage)]
pub struct PlayerStateMachine {
    pub machine: StateMachine,
}

#[derive(Component, Default)]
#[component(HashMapStorage)]
pub struct StartPSM;

// #[derive(Component)]
// #[component(DenseVecStorage)]
// pub struct PlayerStats {
//     pub health: f64,
//     pub damage: f64,
//     pub abilities: Vec<Ability>,
//     pub buffs: Vec<Buff>,
//     pub inventory: [4; [3; Item]],
// }