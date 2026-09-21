use luzure_ecs::{Bundle, Entity, StartupSystem, System};
use luzure_thread::{ThreadError, ThreadMode};

use crate::runtime::RuntimePlan;

use super::Simulation;

pub struct SimulationContext<'a> {
    runtime_plan: &'a mut RuntimePlan,
    simulation: &'a mut Simulation,
}

impl<'a> SimulationContext<'a> {
    pub(crate) const fn new(simulation: &'a mut Simulation, runtime_plan: &'a mut RuntimePlan) -> Self {
        Self {
            runtime_plan,
            simulation,
        }
    }

    pub const fn thread_mode(&self) -> ThreadMode {
        self.runtime_plan.simulation_thread_mode()
    }

    pub const fn set_thread_mode(&mut self, thread_mode: ThreadMode) {
        self.runtime_plan.set_simulation_thread_mode(thread_mode);
    }

    pub const fn tick_rate(&self) -> u32 {
        self.runtime_plan.simulation_tick_rate()
    }

    pub fn set_tick_rate(&mut self, tick_rate: u32) -> Result<(), ThreadError> {
        self.runtime_plan.set_simulation_tick_rate(tick_rate)
    }

    pub fn insert_resource<T: Send + Sync + 'static>(&mut self, resource: T) -> Option<T> {
        self.simulation.world_mut().registry_mut().insert_resource(resource)
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> Entity {
        self.simulation.world_mut().registry_mut().spawn_bundle(bundle)
    }

    pub fn spawn_component<T: Send + Sync + 'static>(&mut self, component: T) -> Entity {
        self.simulation.world_mut().registry_mut().spawn(component)
    }

    pub fn add_startup_system(&mut self, system: StartupSystem) {
        self.simulation.startup_schedule_mut().add_system(system);
    }

    pub fn add_system(&mut self, system: System) {
        self.simulation.schedule_mut().add_system(system);
    }
}
