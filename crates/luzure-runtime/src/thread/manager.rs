use luzure_thread::Thread;

use crate::{runtime::RuntimeError, simulation::SimulationTask};

pub(crate) struct ThreadManager {
    simulation: Option<Thread<SimulationTask>>,
}

impl ThreadManager {
    pub(crate) const fn new() -> Self {
        Self { simulation: None }
    }

    pub(crate) fn start_simulation(&mut self, task: SimulationTask, tick_rate: u32)
        -> Result<(), RuntimeError>
    {
        debug_assert!(self.simulation.is_none());

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
