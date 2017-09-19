use ggez::Context;

use std::fmt::{Debug, Formatter};
use std::fmt::Error;
use std::result::Result;
use std::time::Duration;

use components::*;
use rendering::animation_seq::*;
use resources::*;
use systems::*;

pub enum Trans {
    None,
    Pop,
    Push(Box<State>),
    Switch(Box<State>),
    Quit,
}

pub trait State {
    fn on_start(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) {
    }
    fn on_stop(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) {
    }
    fn on_pause(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) {
    }
    fn on_resume(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) {
    }

    /// Executed on every frame before updating, for use in reacting to events.
    fn handle_events(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) -> Trans {
        Trans::None
    }

    /// Executed repeatedly at stable, predictable intervals (1/60th of a second
    /// by default).
    fn fixed_update(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) -> Trans {
        Trans::None
    }

    /// Executed on every frame immediately, as fast as the engine will allow.
    fn update(
        &mut self,
        _mv: &mut MovingObject,
        _bb: &mut HasAABB,
        _anim: &mut HasAnimationSequence,
        _rend: &mut Renderable,
        _dir: &Directional,
        _pi: &PlayerInput,
        _delta: &DeltaTime,
    ) -> Trans {
        Trans::None
    }
}

unsafe impl Sync for StateMachine {}
unsafe impl Send for StateMachine {}

pub struct StateMachine {
    running: bool,
    state_stack: Vec<Box<State>>,
}

impl StateMachine {
    pub fn new<S>(initial_state: S) -> StateMachine
    where
        S: State + 'static, {
        StateMachine {
            running: false,
            state_stack: vec![Box::new(initial_state)],
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn start(
        &mut self,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        dir: &Directional,
        pi: &PlayerInput,
        delta: &DeltaTime,
    ) {
        if !self.running {
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(mv, bb, anim, rend, dir, pi, delta);
            self.running = true;
        }
    }

    pub fn handle_events(
        &mut self,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        dir: &Directional,
        pi: &PlayerInput,
        delta: &DeltaTime,
    ) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.handle_events(mv, bb, anim, rend, dir, pi, delta),
                None => Trans::None,
            };

            self.transition(trans, mv, bb, anim, rend, dir, pi, delta);
        }
    }

    pub fn fixed_update(
        &mut self,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        dir: &Directional,
        pi: &PlayerInput,
        delta: &DeltaTime,
    ) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.fixed_update(mv, bb, anim, rend, dir, pi, delta),
                None => Trans::None,
            };

            self.transition(trans, mv, bb, anim, rend, dir, pi, delta);
        }
    }

    pub fn update(
        &mut self,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        dir: &Directional,
        pi: &PlayerInput,
        delta: &DeltaTime,
    ) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.update(mv, bb, anim, rend, dir, pi, delta),
                None => Trans::None,
            };

            self.transition(trans, mv, bb, anim, rend, dir, pi, delta);
        }
    }

    fn transition(
        &mut self,
        request: Trans,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        dir: &Directional,
        pi: &PlayerInput,
        delta: &DeltaTime,
    ) {
        if self.running {
            match request {
                Trans::None => (),
                Trans::Pop => self.pop(mv, bb, anim, rend, dir, pi, delta),
                Trans::Push(state) => self.push(state, mv, bb, anim, rend, dir, pi, delta),
                Trans::Switch(state) => self.switch(state, mv, bb, anim, rend, dir, pi, delta),
                Trans::Quit => self.stop(mv, bb, anim, rend, dir, pi, delta),
            }
        }
    }

    fn switch(
        &mut self,
        state: Box<State>,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        dir: &Directional,
        pi: &PlayerInput,
        delta: &DeltaTime,
    ) {
        if self.running {
            if let Some(mut state) = self.state_stack.pop() {
                state.on_stop(mv, bb, anim, rend, dir, pi, delta)
            }

            self.state_stack.push(state);
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(mv, bb, anim, rend, dir, pi, delta);
        }
    }

    fn push(
        &mut self,
        state: Box<State>,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        dir: &Directional,
        pi: &PlayerInput,
        delta: &DeltaTime,
    ) {
        if self.running {
            if let Some(state) = self.state_stack.last_mut() {
                state.on_pause(mv, bb, anim, rend, dir, pi, delta);
            }

            self.state_stack.push(state);
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(mv, bb, anim, rend, dir, pi, delta);
        }
    }

    fn pop(
        &mut self,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        dir: &Directional,
        pi: &PlayerInput,
        delta: &DeltaTime,
    ) {
        if self.running {
            if let Some(mut state) = self.state_stack.pop() {
                state.on_stop(mv, bb, anim, rend, dir, pi, delta);
            }

            if let Some(state) = self.state_stack.last_mut() {
                state.on_resume(mv, bb, anim, rend, dir, pi, delta);
            } else {
                self.running = false;
            }
        }
    }

    fn stop(
        &mut self,
        mv: &mut MovingObject,
        bb: &mut HasAABB,
        anim: &mut HasAnimationSequence,
        rend: &mut Renderable,
        dir: &Directional,
        pi: &PlayerInput,
        delta: &DeltaTime,
    ) {
        if self.running {
            while let Some(mut state) = self.state_stack.pop() {
                state.on_stop(mv, bb, anim, rend, dir, pi, delta);
            }

            self.running = false;
        }
    }
}
