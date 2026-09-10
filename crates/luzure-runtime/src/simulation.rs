mod task;

pub(crate) use task::SimulationTask;

use luzure_ecs::{Schedule, StartupSchedule};
use luzure_world::World;

use std::time::Duration;

pub struct Simulation {
    schedule: Schedule,
    startup_schedule: StartupSchedule,
    world: World,
}

impl Simulation {
    pub const DEFAULT_TICK_RATE: u32 = 32;

    pub(crate) fn new() -> Self {
        Self {
            schedule: Schedule::new(),
            startup_schedule: StartupSchedule::new(),
            world: World::new(),
        }
    }

    pub(crate) fn start(&mut self) {
        self.startup_schedule.run(self.world.registry_mut());
    }

    pub(crate) fn tick(&mut self, delta: Duration) {
        self.schedule.run(self.world.registry_mut(), delta);
    }

    pub const fn schedule(&self) -> &Schedule {
        &self.schedule
    }

    pub const fn schedule_mut(&mut self) -> &mut Schedule {
        &mut self.schedule
    }

    pub const fn world(&self) -> &World {
        &self.world
    }

    pub const fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
