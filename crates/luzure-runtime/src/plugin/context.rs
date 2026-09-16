use luzure_ecs::Registry;
use luzure_game::GameMetadata;

use crate::backend::BackendContext;
use crate::main_thread::{MainContext, MainSchedule};
use crate::render::{RenderContext, RenderExtraction, RenderPlan};
use crate::simulation::{Simulation, SimulationContext, SimulationPlan};
use crate::window::WindowPlan;

pub struct PluginContext<'a> {
    main_registry: &'a mut Registry,
    main_schedule: &'a mut MainSchedule,
    metadata: GameMetadata,
    render_extraction: &'a mut RenderExtraction,
    render_plan: &'a mut RenderPlan,
    simulation: &'a mut Simulation,
    simulation_plan: &'a mut SimulationPlan,
    window_plan: &'a mut WindowPlan,
}

impl<'a> PluginContext<'a> {
    pub(crate) const fn new(metadata: GameMetadata, main_registry: &'a mut Registry, main_schedule: &'a mut MainSchedule, window_plan: &'a mut WindowPlan, render_plan: &'a mut RenderPlan, render_extraction: &'a mut RenderExtraction, simulation: &'a mut Simulation, simulation_plan: &'a mut SimulationPlan)
        -> Self
    {
        Self {
            main_registry,
            main_schedule,
            metadata,
            render_extraction,
            render_plan,
            simulation,
            simulation_plan,
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
        RenderContext::new(self.render_plan, self.render_extraction)
    }

    pub fn simulation(&mut self) -> SimulationContext<'_> {
        SimulationContext::new(self.simulation, self.simulation_plan)
    }

    pub const fn metadata(&self) -> GameMetadata {
        self.metadata
    }
}
