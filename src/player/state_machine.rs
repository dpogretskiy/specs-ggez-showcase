use components::*;
use resources::*;

state_machine!(
    StateMachine; 
    State; 
    _mv: &mut MovingObject,
    _bb: &mut HasAABB,
    _anim: &mut HasAnimationSequence,
    _rend: &mut Renderable,
    _dir: &Directional,
    _pi: &PlayerInput,
    _delta: &DeltaTime
);

#[allow(dead_code)]
struct Dead;
impl State for Dead {
    fn update(&mut self, _: &mut MovingObject, _: &mut HasAABB, _: &mut HasAnimationSequence, _: &mut Renderable, _: &Directional, _: &PlayerInput, _: &DeltaTime) -> Trans {
        Trans::Quit
    }
}

#[allow(dead_code)]
fn funky() {
    let machine = StateMachine::new(Dead);
    machine.is_running();
}