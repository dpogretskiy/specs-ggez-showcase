use components::*;
use player::components::*;
use player::consts as PC;
use rendering::animation_seq::*;
use resources::*;
use specs::*;

pub struct PlayerDirectionSystem;
impl<'a> System<'a> for PlayerDirectionSystem {
    type SystemData = (
        WriteStorage<'a, Directional>,
        ReadStorage<'a, Controlled>,
        Fetch<'a, PlayerInput>,
    );

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

type SMSD<'a> = (
    ReadStorage<'a, Controlled>,
    WriteStorage<'a, PlayerStateMachine>,
    WriteStorage<'a, MovingObject>,
    WriteStorage<'a, HasAABB>,
    WriteStorage<'a, HasAnimationSequence>,
    WriteStorage<'a, Directional>,
    Fetch<'a, PlayerInput>,
    Fetch<'a, DeltaTime>,
);


pub struct PlayerUpdateSystem;
impl<'a> System<'a> for PlayerUpdateSystem {
    type SystemData = SMSD<'a>;

    fn run(&mut self, data: Self::SystemData) {
        use std::cell::RefCell;
        use std::mem;

        let (controlled, mut sm, mut mv, mut bb, mut anim, dir, input, time) = data;

        for (_, sm, mv, bb, anim, dir) in
            (&controlled, &mut sm, &mut mv, &mut bb, &mut anim, &dir).join()
        {
            let mvrc = RefCell::new(mv.clone());
            let bbrc = RefCell::new(bb.clone());
            let animrc = RefCell::new(anim.clone());
            sm.machine.update(&mut (
                mvrc.clone(),
                bbrc.clone(),
                animrc.clone(),
                (*dir).clone(),
                (*input).clone(),
                (*time).clone(),
            ));

            mem::swap(mv, &mut mvrc.into_inner());
            mem::swap(bb, &mut bbrc.into_inner());
            mem::swap(anim, &mut animrc.into_inner())
        }
    }
}

pub struct PlayerFixedUpdateSystem;
impl<'a> System<'a> for PlayerFixedUpdateSystem {
    type SystemData = SMSD<'a>;

    fn run(&mut self, data: Self::SystemData) {
        use std::cell::RefCell;
        use std::mem;

        let (controlled, mut sm, mut mv, mut bb, mut anim, dir, input, time) = data;

        for (_, sm, mv, bb, anim, dir) in
            (&controlled, &mut sm, &mut mv, &mut bb, &mut anim, &dir).join()
        {
            let mvrc = RefCell::new(mv.clone());
            let bbrc = RefCell::new(bb.clone());
            let animrc = RefCell::new(anim.clone());
            sm.machine.fixed_update(&mut (
                mvrc.clone(),
                bbrc.clone(),
                animrc.clone(),
                (*dir).clone(),
                (*input).clone(),
                (*time).clone(),
            ));

            mem::swap(mv, &mut mvrc.into_inner());
            mem::swap(bb, &mut bbrc.into_inner());
            mem::swap(anim, &mut animrc.into_inner())
        }
    }
}

pub struct PlayerHandleEventsSystem;
impl<'a> System<'a> for PlayerHandleEventsSystem {
    type SystemData = SMSD<'a>;

    fn run(&mut self, data: Self::SystemData) {
        use std::cell::RefCell;
        use std::mem;

        let (controlled, mut sm, mut mv, mut bb, mut anim, dir, input, time) = data;

        for (_, sm, mv, bb, anim, dir) in
            (&controlled, &mut sm, &mut mv, &mut bb, &mut anim, &dir).join()
        {
            let mvrc = RefCell::new(mv.clone());
            let bbrc = RefCell::new(bb.clone());
            let animrc = RefCell::new(anim.clone());
            sm.machine.handle_events(&mut (
                mvrc.clone(),
                bbrc.clone(),
                animrc.clone(),
                (*dir).clone(),
                (*input).clone(),
                (*time).clone(),
            ));

            mem::swap(mv, &mut mvrc.into_inner());
            mem::swap(bb, &mut bbrc.into_inner());
            mem::swap(anim, &mut animrc.into_inner())
        }
    }
}

pub struct StartPSMSystem;
impl<'a> System<'a> for StartPSMSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, StartPSM>,
        ReadStorage<'a, Controlled>,
        WriteStorage<'a, PlayerStateMachine>,
        WriteStorage<'a, MovingObject>,
        WriteStorage<'a, HasAABB>,
        WriteStorage<'a, HasAnimationSequence>,
        WriteStorage<'a, Directional>,
        Fetch<'a, PlayerInput>,
        Fetch<'a, DeltaTime>);


    fn run(&mut self, data: Self::SystemData) {
        use std::cell::RefCell;
        use std::mem;
        let (e, mut start, controlled, mut sm, mut mv, mut bb, mut anim, dir, input, time) = data;

        for s in start.join() {
            println!("Start!");
        }

        let mut rem = vec![];
        
        for (e, _, _, sm, mv, bb, anim, dir) in
            (&*e, &mut start, &controlled, &mut sm, &mut mv, &mut bb, &mut anim, &dir).join()
        {
            println!("Starting some!: {:?}", e);
            let mvrc = RefCell::new(mv.clone());
            let bbrc = RefCell::new(bb.clone());
            let animrc = RefCell::new(anim.clone());
            sm.machine.start(&mut (
                mvrc.clone(),
                bbrc.clone(),
                animrc.clone(),
                (*dir).clone(),
                (*input).clone(),
                (*time).clone(),
            ));

            mem::swap(mv, &mut mvrc.into_inner());
            mem::swap(bb, &mut bbrc.into_inner());
            mem::swap(anim, &mut animrc.into_inner());
            rem.push(e);
        }

        for e in rem.iter() {
            start.remove(e.clone());
        }
    }
}

pub struct PlayerAux;
impl PlayerAux {
    pub fn movement(mv: &mut MovingObject, bb: &HasAABB, direction: &Directional) {
        match *direction {
            Directional::Left => if bb.pushes_left_wall {
                PlayerAux::stop(mv);
            } else {
                mv.accel.x = -PC::WALK_ACCEL;
                mv.velocity.x = (-PC::WALK_SPEED / 2.0).min(mv.velocity.x).max(-PC::WALK_SPEED);
            },
            Directional::Right => if bb.pushes_right_wall {
                PlayerAux::stop(mv);
            } else {
                mv.accel.x = PC::WALK_ACCEL;
                mv.velocity.x = (PC::WALK_SPEED / 2.0).max(mv.velocity.x).min(PC::WALK_SPEED);
            },
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
