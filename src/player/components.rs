use specs::*;

#[derive(Debug, Component, Default)]
#[component(NullStorage)]
pub struct Controlled;

#[derive(Debug, Component, Default)]
#[component(NullStorage)]
pub struct IdlePlayer;

#[derive(Debug, Component, Default)]
#[component(NullStorage)]
pub struct RunningPlayer;

#[derive(Debug, Component, Default)]
#[component(NullStorage)]
pub struct JumpingPlayer;

#[derive(Debug, Component, Default)]
#[component(NullStorage)]
pub struct SlidingPlayer;

#[derive(Debug, Component, Default)]
#[component(NullStorage)]
pub struct AttackingPlayer;
