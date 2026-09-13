use luzure_thread::Thread;

use crate::{runtime::RuntimeError, simulation::SimulationTask, thread::ThreadPlan};

pub(crate) struct ThreadManager {
    plan: ThreadPlan,
    simulation: Option<Thread<SimulationTask>>,
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

        let tick_rate = self.plan.simulation().tick_rate();

        self.simulation = Some(Thread::spawn("luzure-simulation", task, tick_rate)?);

        Ok(())
    }

    pub(crate) fn take_error(&mut self) -> Option<RuntimeError> {
        self.simulation.as_mut()?
            .take_error()
            .map(Into::into)
    }

    pub(crate) fn stop(&mut self) -> Option<RuntimeError> {
        let thread = self.simulation.take()?;

        match thread.stop() {
            Ok((_, Some(error))) => Some(error.into()),
            Ok((_, None)) => None,
            Err(error) => Some(error.into()),
        }
    }
}
