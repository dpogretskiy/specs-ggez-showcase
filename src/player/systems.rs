use components::*;
use player::consts as PC;
use resources::*;
use specs::*;

pub struct PlayerDirectionSystem;
impl<'a> System<'a> for PlayerDirectionSystem {
    type SystemData = (WriteStorage<'a, Directional>,
     ReadStorage<'a, Controlled>,
     Fetch<'a, PlayerInput>);

    fn run(&mut self, (mut directional, controlled, input): Self::SystemData) {
        for (mut dir, _) in (&mut directional, &controlled).join() {
            if input.left ^ input.right {
                if input.left {
                    *dir = Directional::Left;
                } else {
                    *dir = Directional::Right;
                }
            }
        }
    }
}

type SMSD<'a> = (ReadStorage<'a, Controlled>,
                 WriteStorage<'a, PlayerStateMachine>,
                 WriteStorage<'a, MovingObject>,
                 WriteStorage<'a, HasAABB>,
                 WriteStorage<'a, HasAnimationSequence>,
                 WriteStorage<'a, Renderable>,
                 WriteStorage<'a, Directional>,
                 Fetch<'a, PlayerInput>,
                 Fetch<'a, DeltaTime>);


pub struct PlayerUpdateSystem;
impl<'a> System<'a> for PlayerUpdateSystem {
    type SystemData = SMSD<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (controlled, mut sm, mut mv, mut bb, mut anim, mut rend, dir, input, time) = data;

        for (_, sm, mv, bb, anim, rend, dir) in
            (
                &controlled,
                &mut sm,
                &mut mv,
                &mut bb,
                &mut anim,
                &mut rend,
                &dir,
            ).join()
        {
            sm.machine.update(mv, bb, anim, rend, dir, &*input, &*time);
        }
    }
}

pub struct PlayerFixedUpdateSystem;
impl<'a> System<'a> for PlayerFixedUpdateSystem {
    type SystemData = SMSD<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (controlled, mut sm, mut mv, mut bb, mut anim, mut rend, dir, input, time) = data;

        for (_, sm, mv, bb, anim, rend, dir) in
            (
                &controlled,
                &mut sm,
                &mut mv,
                &mut bb,
                &mut anim,
                &mut rend,
                &dir,
            ).join()
        {
            sm.machine.fixed_update(mv, bb, anim, rend, dir, &*input, &*time);
        }
    }
}

pub struct PlayerHandleEventsSystem;
impl<'a> System<'a> for PlayerHandleEventsSystem {
    type SystemData = SMSD<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (controlled, mut sm, mut mv, mut bb, mut anim, mut rend, dir, input, time) = data;

        for (_, sm, mv, bb, anim, rend, dir) in
            (
                &controlled,
                &mut sm,
                &mut mv,
                &mut bb,
                &mut anim,
                &mut rend,
                &dir,
            ).join()
        {
            sm.machine.handle_events(mv, bb, anim, rend, dir, &*input, &*time);
        }
    }
}

pub struct StartPSMSystem;
impl<'a> System<'a> for StartPSMSystem {
    type SystemData = (Entities<'a>,
     WriteStorage<'a, StartPSM>,
     ReadStorage<'a, Controlled>,
     WriteStorage<'a, PlayerStateMachine>,
     WriteStorage<'a, MovingObject>,
     WriteStorage<'a, HasAABB>,
     WriteStorage<'a, HasAnimationSequence>,
     WriteStorage<'a, Renderable>,
     WriteStorage<'a, Directional>,
     Fetch<'a, PlayerInput>,
     Fetch<'a, DeltaTime>);

    fn run(&mut self, data: Self::SystemData) {
        let (e,
             mut start,
             controlled,
             mut sm,
             mut mv,
             mut bb,
             mut anim,
             mut rend,
             dir,
             input,
             time) = data;

        let mut rem = vec![];

        for (e, _, _, sm, mv, bb, anim, rend, dir) in
            (
                &*e,
                &mut start,
                &controlled,
                &mut sm,
                &mut mv,
                &mut bb,
                &mut anim,
                &mut rend,
                &dir,
            ).join()
        {
            sm.machine.start(mv, bb, anim, rend, dir, &*input, &*time);
            rem.push(e);
        }

        for e in rem.iter() {
            start.remove(e.clone());
        }
    }
}

pub struct ResetInputSystem;
impl<'a> System<'a> for ResetInputSystem {
    type SystemData = FetchMut<'a, PlayerInput>;

    fn run(&mut self, mut input: Self::SystemData) {
        input.reset_actions();
    }
}

pub struct PlayerAux;
impl PlayerAux {
    pub fn movement(mv: &mut MovingObject, bb: &HasAABB, direction: &Directional) {
        match *direction {
            Directional::Left => {
                if bb.pushes_left_wall {
                    PlayerAux::stop(mv);
                } else {
                    mv.accel.x = -PC::WALK_ACCEL;
                    mv.velocity.x = (-PC::WALK_SPEED / 2.0).min(mv.velocity.x).max(-PC::WALK_SPEED);
                }
            }
            Directional::Right => {
                if bb.pushes_right_wall {
                    PlayerAux::stop(mv);
                } else {
                    mv.accel.x = PC::WALK_ACCEL;
                    mv.velocity.x = (PC::WALK_SPEED / 2.0).max(mv.velocity.x).min(PC::WALK_SPEED);
                }
            }
        }
    }

    pub fn stop(mv: &mut MovingObject) {
        mv.accel.x = 0.0;
        mv.velocity.x = 0.0;
    }

    pub fn slow_down(mv: &mut MovingObject, fast: bool) {
        if fast {
            if mv.velocity.x > PC::WALK_ACCEL / 2.0 {
                mv.accel.x = -PC::WALK_ACCEL * 5.0;
            } else if mv.velocity.x < -PC::WALK_ACCEL / 2.0 {
                mv.accel.x = PC::WALK_ACCEL * 5.0;
            } else {
                PlayerAux::stop(mv);
            }
        } else {
            mv.accel.x = -mv.velocity.x / 2.0;
        }
    }
}
