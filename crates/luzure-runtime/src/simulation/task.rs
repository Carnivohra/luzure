mod scene;

use luzure_render::render::RenderError;
use luzure_render::RenderScene;
use luzure_thread::ThreadTask;

use std::time::Duration;

use crate::render::{RenderExtraction, RenderSceneProducer};

use scene::SimulationTaskScene;

use super::Simulation;

pub(crate) struct SimulationTask {
    render_extraction: RenderExtraction,
    render_scene: SimulationTaskScene,
    simulation: Simulation,
}

impl SimulationTask {
    pub(crate) fn direct(simulation: Simulation, render_extraction: RenderExtraction) -> Self {
        Self {
            render_extraction,
            render_scene: SimulationTaskScene::direct(),
            simulation,
        }
    }

    pub(crate) fn triple_buffered(simulation: Simulation, render_extraction: RenderExtraction, render_scene: RenderSceneProducer) -> Self {
        Self {
            render_extraction,
            render_scene: SimulationTaskScene::triple_buffered(render_scene),
            simulation,
        }
    }

    pub(crate) const fn render_scene(&self) -> Option<&RenderScene> {
        self.render_scene.current()
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

        let render_extraction = &mut self.render_extraction;
        let registry = self.simulation.world().registry();

        self.render_scene.publish(|scene| {
            scene.clear();

            render_extraction.run(registry, scene)
        })
    }
}
