use luzure_thread::ThreadTask;
use luzure_world::World;

use std::time::Duration;

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub const DEFAULT_TICK_RATE: u32 = 60;

    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub const fn world(&self) -> &World {
        &self.world
    }

    pub const fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}

impl ThreadTask for Simulation {
    fn tick(&mut self, _delta: Duration) {}
}
