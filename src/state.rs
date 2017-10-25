

use engine:Engine;


pub enum Trans{
    None,
    Pop,
    Push(Box<State>),
    Switch(Box<State>),
    Quit,
}

pub trait State{
    fn on_start(&mut self, _eng: &mut Engine) {}
       /// Executed when the game state exits.
    fn on_stop(&mut self, _eng: &mut Engine) {}

    /// Executed when a different game state is pushed onto the stack.
    fn on_pause(&mut self, _eng: &mut Engine) {}

    /// Executed when the application returns to this game state once again.
    fn on_resume(&mut self, _eng: &mut Engine) {}

    /// Executed on every frame before updating, for use in reacting to events.
    fn handle_event(&mut self, _eng: &mut Engine, _event: Event) -> Trans {
        Trans::None
    }

    /// Executed repeatedly at stable, predictable intervals (1/60th of a second
    /// by default).
    fn fixed_update(&mut self, _eng: &mut Engine) -> Trans {
        Trans::None
    }

    /// Executed on every frame immediately, as fast as the engine will allow.
    fn update(&mut self, _eng: &mut Engine) -> Trans {
        Trans::None
    }
}