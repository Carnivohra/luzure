use luzure_ecs::{Schedule, System};
use luzure_world::World;

use crate::render::{RenderExtractSystem, RenderExtraction};

pub struct PluginContext<'a> {
    render_extraction: &'a mut RenderExtraction,
    schedule: &'a mut Schedule,
    world: &'a mut World,
}

impl<'a> PluginContext<'a> {
    pub(crate) const fn new(render_extraction: &'a mut RenderExtraction, schedule: &'a mut Schedule, world: &'a mut World)
        -> Self
    {
        Self {
            render_extraction,
            schedule,
            world,
        }
    }

    pub fn add_system(&mut self, system: System) {
        self.schedule.add_system(system);
    }

    pub fn add_render_extract_system(&mut self, system: RenderExtractSystem) {
        self.render_extraction.add_system(system);
    }

    pub const fn world(&self) -> &World {
        self.world
    }

    pub const fn world_mut(&mut self) -> &mut World {
        self.world
    }
}
