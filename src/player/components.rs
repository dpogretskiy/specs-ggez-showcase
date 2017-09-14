use player::state_machine::StateMachine;
use player::systems::PlayerSystemData;
use specs::*;

#[derive(Debug, Component, Default)]
#[component(NullStorage)]
pub struct Controlled;

#[derive(Component)]
#[component(HashMapStorage)]
pub struct PlayerStateMachine {
    machine: StateMachine<PlayerSystemData<'static>>,
}
