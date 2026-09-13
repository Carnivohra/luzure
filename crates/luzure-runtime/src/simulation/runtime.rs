use luzure_thread::TaskRunner;

use crate::runtime::RuntimeError;

use super::{SimulationPlan, SimulationTask};

pub(crate) struct SimulationRuntime {
    plan: SimulationPlan,
    runner: Option<TaskRunner<SimulationTask>>,
}

impl SimulationRuntime {
    pub(crate) const fn new() -> Self {
        Self {
            plan: SimulationPlan::new(),
            runner: None,
        }
    }

    pub(crate) const fn plan_mut(&mut self) -> &mut SimulationPlan {
        &mut self.plan
    }

    pub(crate) fn start(&mut self, task: SimulationTask) -> Result<(), RuntimeError> {
        debug_assert!(self.runner.is_none());

        self.runner = Some(TaskRunner::start(
            "luzure-simulation",
            task,
            self.plan.thread_mode(),
            self.plan.tick_rate(),
        )?);

        Ok(())
    }

    pub(crate) fn update(&mut self) -> Result<(), RuntimeError> {
        let Some(runner) = &mut self.runner else {
            return Ok(());
        };

        runner.update()?;

        Ok(())
    }

    pub(crate) fn suspend(&mut self) {
        if let Some(runner) = &mut self.runner {
            runner.pause();
        }
    }

    pub(crate) fn resume(&mut self) {
        if let Some(runner) = &mut self.runner {
            runner.resume();
        }
    }

    pub(crate) fn stop(&mut self) -> Option<RuntimeError> {
        let runner = self.runner.take()?;

        match runner.stop() {
            Ok((_, Some(error))) => Some(error.into()),
            Ok((_, None)) => None,
            Err(error) => Some(error.into()),
        }
    }
}
