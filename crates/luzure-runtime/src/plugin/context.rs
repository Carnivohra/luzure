use luzure_ecs::{Schedule, StartupSchedule, StartupSystem, System};

use crate::render::{RenderExtractSystem, RenderExtraction};

pub struct PluginContext<'a> {
    render_extraction: &'a mut RenderExtraction,
    schedule: &'a mut Schedule,
    startup_schedule: &'a mut StartupSchedule,
}

impl<'a> PluginContext<'a> {
    pub(crate) const fn new(render_extraction: &'a mut RenderExtraction, schedule: &'a mut Schedule, startup_schedule: &'a mut StartupSchedule)
        -> Self
    {
        Self {
            render_extraction,
            schedule,
            startup_schedule,
        }
    }

    pub fn add_startup_system(&mut self, system: StartupSystem) {
        self.startup_schedule.add_system(system);
    }

    pub fn add_system(&mut self, system: System) {
        self.schedule.add_system(system);
    }

    pub fn add_render_extract_system(&mut self, system: RenderExtractSystem) {
        self.render_extraction.add_system(system);
    }
}
