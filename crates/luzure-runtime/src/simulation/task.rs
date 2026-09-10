use luzure_render::render::RenderError;
use luzure_thread::ThreadTask;

use std::time::Duration;

use crate::render::{RenderExtraction, RenderSceneProducer};

use super::Simulation;

pub(crate) struct SimulationTask {
    render_extraction: RenderExtraction,
    render_scenes: RenderSceneProducer,
    simulation: Simulation,
}

impl SimulationTask {
    pub(crate) fn new(simulation: Simulation, render_extraction: RenderExtraction, render_scenes: RenderSceneProducer) -> Self {
        Self {
            render_extraction,
            render_scenes,
            simulation,
        }
    }
}

impl ThreadTask for SimulationTask {
    type Error = RenderError;

    fn start(&mut self) -> Result<(), Self::Error> {
        self.simulation.start();

        Ok(())
    }

    fn tick(&mut self, delta: Duration) -> Result<(), Self::Error> {
        self.simulation.tick(delta);

        let render_extraction = &self.render_extraction;
        let registry = self.simulation.world().registry();

        self.render_scenes.publish(|scene| {
            scene.clear();

            render_extraction.run(registry, scene)
        })
    }
}
