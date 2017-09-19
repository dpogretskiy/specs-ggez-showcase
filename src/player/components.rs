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
