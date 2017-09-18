use player::state_machine::StateMachine;
use player::state::PlayerData;
use specs::*;

#[derive(Debug, Component, Default)]
#[component(NullStorage)]
pub struct Controlled;

#[derive(Component)]
#[component(HashMapStorage)]
pub struct PlayerStateMachine {
    pub machine: StateMachine<PlayerData>,
}

#[derive(Component, Default)]
#[component(HashMapStorage)]
pub struct StartPSM;