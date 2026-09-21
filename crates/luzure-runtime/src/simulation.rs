mod context;
mod runtime;
mod task;

pub use context::SimulationContext;
pub use luzure_thread::ThreadMode;
pub(crate) use runtime::SimulationRuntime;
pub(crate) use task::SimulationTask;

use luzure_ecs::{Schedule, StartupSchedule};
use luzure_world::World;

use std::time::Duration;

pub(crate) struct Simulation {
    schedule: Schedule,
    startup_schedule: StartupSchedule,
    world: World,
}

impl Simulation {
    pub(crate) const DEFAULT_TICK_RATE: u32 = 32;

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

    pub(crate) const fn schedule_mut(&mut self) -> &mut Schedule {
        &mut self.schedule
    }

    pub(crate) const fn startup_schedule_mut(&mut self) -> &mut StartupSchedule {
        &mut self.startup_schedule
    }

    pub(crate) const fn world(&self) -> &World {
        &self.world
    }

    pub(crate) const fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
