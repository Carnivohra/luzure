use luzure_render::render::RenderError;
use luzure_thread::ThreadTask;

use std::time::Duration;

use crate::{plugin::PluginContext, render::{RenderExtraction, RenderWriter}};

use super::Simulation;

pub(crate) struct SimulationTask {
    render_extraction: RenderExtraction,
    render_writer: RenderWriter,
    simulation: Simulation,
}

impl SimulationTask {
    pub(crate) fn new(simulation: Simulation, render_writer: RenderWriter) -> Self {
        Self {
            render_extraction: RenderExtraction::new(),
            render_writer,
            simulation,
        }
    }

    pub(crate) fn plugin_context(&mut self) -> PluginContext<'_> {
        PluginContext::new(
            &mut self.render_extraction,
            &mut self.simulation.schedule,
            &mut self.simulation.world,
        )
    }
}

impl ThreadTask for SimulationTask {
    type Error = RenderError;

    fn tick(&mut self, delta: Duration) -> Result<(), Self::Error> {
        self.simulation.tick(delta);

        let scene = self.render_writer.scene_mut();

        scene.clear();
        self.render_extraction.run(self.simulation.world().registry(), scene)?;
        self.render_writer.publish();

        Ok(())
    }
}
