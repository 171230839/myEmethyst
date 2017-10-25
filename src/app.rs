
use engine::Engine;
use state::State;

use ecs::{Component, Dispatcher, DispatcherBuilder, System, World};
use ecs::common::Errors;
use winit::{Event, WindowEvent};
use shrev::{EventChannel, ReaderId};


pub struct Application{
   // pub engine: Engine,
}

impl Application{
    pub fn new<P,S>(path: P, initial_state: S) ->
    where P: AsRef<Path>, S: State,
    {
        ApplicationBuilder::new(path, initial_state)?.build()
    }
}


struct ApplicationBuilder{

}

impl ApplicationBuilder{
    pub fn new<P: AsRef<Path>>(path: P, initial_state: T) -> Result<Self>{
        use rayon::Configuration;

        println!("Initializing Amethyst...");
        println!("Version: {}", vergen::semver());
        println!("Platform: {}", vergen::target());
        println!("Git commit: {}", vergen::sha());
        let cfg = Configuration::new();
        let pool = ThreadPool::new(cfg)
            .map(|p| Arc::new(p))
            .map_err(|_| Error::Application)?;
         let mut world = World::new();
        world.add_resource(Loader::new(path.as_ref(), pool.clone()));
         let events = EventChannel::<Event>::with_capacity(2000);
    }
}