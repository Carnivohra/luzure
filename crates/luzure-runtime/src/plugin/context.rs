use luzure_ecs::Registry;
use luzure_game::GameMetadata;

use crate::backend::BackendContext;
use crate::main::{MainContext, MainSchedule};
use crate::render::{RenderContext, RenderExtraction, RenderPlan};
use crate::runtime::RuntimePlan;
use crate::simulation::{Simulation, SimulationContext};
use crate::window::WindowPlan;

pub struct PluginContext<'a> {
    main_registry: &'a mut Registry,
    main_schedule: &'a mut MainSchedule,
    metadata: GameMetadata,
    render_extraction: &'a mut RenderExtraction,
    render_plan: &'a mut RenderPlan,
    runtime_plan: &'a mut RuntimePlan,
    simulation: &'a mut Simulation,
    window_plan: &'a mut WindowPlan,
}

impl<'a> PluginContext<'a> {
    pub(crate) const fn new(metadata: GameMetadata, main_registry: &'a mut Registry, main_schedule: &'a mut MainSchedule, window_plan: &'a mut WindowPlan, render_plan: &'a mut RenderPlan, render_extraction: &'a mut RenderExtraction, runtime_plan: &'a mut RuntimePlan, simulation: &'a mut Simulation)
        -> Self
    {
        Self {
            main_registry,
            main_schedule,
            metadata,
            render_extraction,
            render_plan,
            runtime_plan,
            simulation,
            window_plan,
        }
    }

    pub fn backend(&mut self) -> BackendContext<'_> {
        BackendContext::new(self.main_registry, self.window_plan)
    }

    pub fn main(&mut self) -> MainContext<'_> {
        MainContext::new(self.main_registry, self.main_schedule)
    }

    pub fn render(&mut self) -> RenderContext<'_> {
        RenderContext::new(self.render_plan, self.render_extraction, self.runtime_plan)
    }

    pub fn simulation(&mut self) -> SimulationContext<'_> {
        SimulationContext::new(self.simulation, self.runtime_plan)
    }

    pub const fn metadata(&self) -> GameMetadata {
        self.metadata
    }
}
