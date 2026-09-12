use luzure_ecs::Registry;
use luzure_game::GameMetadata;

use crate::backend::BackendContext;
use crate::render::{RenderContext, RenderExtraction, RenderPlan};
use crate::simulation::{Simulation, SimulationContext};
use crate::window::WindowPlan;

pub struct PluginContext<'a> {
    pub backend: BackendContext<'a>,
    pub render: RenderContext<'a>,
    pub simulation: SimulationContext<'a>,
    metadata: GameMetadata,
}

impl<'a> PluginContext<'a> {
    pub(crate) const fn new(metadata: GameMetadata, runtime_registry: &'a mut Registry, window_plan: &'a mut WindowPlan, render_plan: &'a mut RenderPlan, render_extraction: &'a mut RenderExtraction, simulation: &'a mut Simulation)
        -> Self
    {
        Self {
            backend: BackendContext::new(runtime_registry, window_plan),
            render: RenderContext::new(render_plan, render_extraction),
            simulation: SimulationContext::new(simulation),
            metadata,
        }
    }

    pub const fn metadata(&self) -> GameMetadata {
        self.metadata
    }
}
