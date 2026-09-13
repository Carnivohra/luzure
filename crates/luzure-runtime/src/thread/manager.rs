use crate::{runtime::RuntimeError, simulation::SimulationTask, thread::{ThreadPlan, runner::SimulationRunner}};

pub(crate) struct ThreadManager {
    plan: ThreadPlan,
    simulation: Option<SimulationRunner>,
}

impl ThreadManager {
    pub(crate) const fn new() -> Self {
        Self {
            plan: ThreadPlan::new(),
            simulation: None,
        }
    }

    pub(crate) const fn plan_mut(&mut self) -> &mut ThreadPlan {
        &mut self.plan
    }

    pub(crate) fn start_simulation(&mut self, task: SimulationTask)
        -> Result<(), RuntimeError>
    {
        debug_assert!(self.simulation.is_none());

        let mode = self.plan.simulation().mode();
        let tick_rate = self.plan.simulation().tick_rate();

        self.simulation = Some(SimulationRunner::start(task, mode, tick_rate)?);

        Ok(())
    }

    pub(crate) fn update(&mut self) -> Result<(), RuntimeError> {
        let Some(simulation) = &mut self.simulation else {
            return Ok(());
        };

        simulation.update()
    }

    pub(crate) fn suspend(&mut self) {
        if let Some(simulation) = &mut self.simulation {
            simulation.suspend();
        }
    }

    pub(crate) fn resume(&mut self) {
        if let Some(simulation) = &mut self.simulation {
            simulation.resume();
        }
    }

    pub(crate) fn stop(&mut self) -> Option<RuntimeError> {
        self.simulation.take()?.stop()
    }
}
