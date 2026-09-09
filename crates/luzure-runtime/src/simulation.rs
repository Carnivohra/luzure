use luzure_ecs::Schedule;
use luzure_thread::ThreadTask;
use luzure_world::World;

use std::time::Duration;

use crate::render::RenderWriter;

pub struct Simulation {
    render_writer: RenderWriter,
    schedule: Schedule,
    world: World,
}

impl Simulation {
    pub const DEFAULT_TICK_RATE: u32 = 60;

    pub(crate) fn new(render_writer: RenderWriter) -> Self {
        Self {
            render_writer,
            schedule: Schedule::new(),
            world: World::new(),
        }
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

impl ThreadTask for Simulation {
    fn tick(&mut self, delta: Duration) {
        self.render_writer.scene_mut().clear();
        self.schedule.run(self.world.registry_mut(), delta);
        self.render_writer.publish();
    }
}
