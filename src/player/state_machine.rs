use ggez::Context;

use std::fmt::{Debug, Formatter};
use std::fmt::Error;
use std::result::Result;
use std::time::Duration;

pub enum Trans<T> {
    None,
    Pop,
    Push(Box<State<T>>),
    Switch(Box<State<T>>),
    Quit,
}

pub trait State<T> {
    fn on_start(&mut self, _data: &mut T) {}
    fn on_stop(&mut self, _data: &mut T) {}
    fn on_pause(&mut self, _data: &mut T) {}
    fn on_resume(&mut self, _data: &mut T) {}

    /// Executed on every frame before updating, for use in reacting to events.
    fn handle_events(&mut self, _data: &mut T) -> Trans<T> {
        Trans::None
    }

    /// Executed repeatedly at stable, predictable intervals (1/60th of a second
    /// by default).
    fn fixed_update(&mut self, _data: &mut T) -> Trans<T> {
        Trans::None
    }

    /// Executed on every frame immediately, as fast as the engine will allow.
    fn update(&mut self, _data: &mut T) -> Trans<T> {
        Trans::None
    }
}

pub struct StateMachine<T>
where
    T: Send + Sync, {
    running: bool,
    state_stack: Vec<Box<State<T>>>,
}

unsafe impl<T> Sync for StateMachine<T>
where
    T: Send + Sync,
{
}

unsafe impl<T> Send for StateMachine<T>
where
    T: Send + Sync,
{
}

impl<T> StateMachine<T>
where
    T: Send + Sync, {
    pub fn new<S>(initial_state: S) -> StateMachine<T>
    where
        S: State<T> + 'static, {
        StateMachine {
            running: false,
            state_stack: vec![Box::new(initial_state)],
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn start(&mut self, data: &mut T) {
        if !self.running {
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(data);
            self.running = true;
        }
    }

    pub fn handle_events(&mut self, data: &mut T) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.handle_events(data),
                None => Trans::None,
            };

            self.transition(trans, data);
        }
    }

    pub fn fixed_update(&mut self, data: &mut T) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.fixed_update(data),
                None => Trans::None,
            };

            self.transition(trans, data);
        }
    }

    pub fn update(&mut self, data: &mut T) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.update(data),
                None => Trans::None,
            };

            self.transition(trans, data);
        }
    }

    fn transition(&mut self, request: Trans<T>, data: &mut T) {
        if self.running {
            match request {
                Trans::None => (),
                Trans::Pop => self.pop(data),
                Trans::Push(state) => self.push(state, data),
                Trans::Switch(state) => self.switch(state, data),
                Trans::Quit => self.stop(data),
            }
        }
    }

    fn switch(&mut self, state: Box<State<T>>, data: &mut T) {
        if self.running {
            if let Some(mut state) = self.state_stack.pop() {
                state.on_stop(data)
            }

            self.state_stack.push(state);
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(data);
        }
    }

    fn push(&mut self, state: Box<State<T>>, data: &mut T) {
        if self.running {
            if let Some(state) = self.state_stack.last_mut() {
                state.on_pause(data);
            }

            self.state_stack.push(state);
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(data);
        }
    }

    fn pop(&mut self, data: &mut T) {
        if self.running {
            if let Some(mut state) = self.state_stack.pop() {
                state.on_stop(data);
            }

            if let Some(state) = self.state_stack.last_mut() {
                state.on_resume(data);
            } else {
                self.running = false;
            }
        }
    }

    fn stop(&mut self, data: &mut T) {
        if self.running {
            while let Some(mut state) = self.state_stack.pop() {
                state.on_stop(data);
            }

            self.running = false;
        }
    }
}
