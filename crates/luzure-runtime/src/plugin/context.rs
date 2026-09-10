mod backend;
mod render;
mod simulation;

pub use backend::BackendContext;
pub use render::RenderContext;
pub use simulation::SimulationContext;

use luzure_ecs::Registry;
use luzure_game::GameMetadata;

use crate::render::RenderExtraction;
use crate::simulation::Simulation;
use crate::window::WindowPlan;

pub struct PluginContext<'a> {
    pub backend: BackendContext<'a>,
    pub render: RenderContext<'a>,
    pub simulation: SimulationContext<'a>,
    metadata: GameMetadata,
}

impl<'a> PluginContext<'a> {
    pub(crate) const fn new(metadata: GameMetadata, backend_registry: &'a mut Registry, windows: &'a mut WindowPlan, render_extraction: &'a mut RenderExtraction, simulation: &'a mut Simulation)
        -> Self
    {
        Self {
            backend: BackendContext::new(backend_registry, windows),
            render: RenderContext::new(render_extraction),
            simulation: SimulationContext::new(simulation),
            metadata,
        }
    }

    pub const fn metadata(&self) -> GameMetadata {
        self.metadata
    }
}
