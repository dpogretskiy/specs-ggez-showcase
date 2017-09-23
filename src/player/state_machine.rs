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
