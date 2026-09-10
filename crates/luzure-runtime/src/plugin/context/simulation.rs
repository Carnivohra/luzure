use luzure_ecs::{StartupSystem, System};

use crate::simulation::Simulation;

pub struct SimulationContext<'a> {
    simulation: &'a mut Simulation,
}

impl<'a> SimulationContext<'a> {
    pub(crate) const fn new(simulation: &'a mut Simulation) -> Self {
        Self { simulation }
    }

    pub fn insert_resource<T: Send + Sync + 'static>(&mut self, resource: T) -> Option<T> {
        self.simulation.world_mut().registry_mut().insert_resource(resource)
    }

    pub fn add_startup_system(&mut self, system: StartupSystem) {
        self.simulation.startup_schedule_mut().add_system(system);
    }

    pub fn add_system(&mut self, system: System) {
        self.simulation.schedule_mut().add_system(system);
    }
}
