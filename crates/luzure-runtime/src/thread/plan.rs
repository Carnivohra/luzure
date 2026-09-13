mod simulation;

pub use simulation::SimulationThreadPlan;

pub(crate) struct ThreadPlan {
    simulation: SimulationThreadPlan,
}

impl ThreadPlan {
    pub(crate) const fn new() -> Self {
        Self {
            simulation: SimulationThreadPlan::new(),
        }
    }

    pub(crate) const fn simulation(&self) -> &SimulationThreadPlan {
        &self.simulation
    }

    pub(crate) const fn simulation_mut(&mut self) -> &mut SimulationThreadPlan {
        &mut self.simulation
    }
}
