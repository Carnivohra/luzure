use luzure_thread::Thread;

use crate::{runtime::RuntimeError, simulation::SimulationTask, thread::{ThreadMode, local::LocalSimulation}};

pub(crate) enum SimulationRunner {
    MainThread(Box<LocalSimulation>),
    Threaded(Thread<SimulationTask>),
}

impl SimulationRunner {
    pub(crate) fn start(task: SimulationTask, mode: ThreadMode, tick_rate: u32)
        -> Result<Self, RuntimeError>
    {
        match mode {
            ThreadMode::Automatic => Self::automatic(task, tick_rate),
            ThreadMode::MainThread => Ok(Self::MainThread(Box::new(LocalSimulation::start(task, tick_rate)?))),
            ThreadMode::Threaded => Self::threaded(task, tick_rate),
        }
    }

    pub(crate) fn update(&mut self) -> Result<(), RuntimeError> {
        match self {
            Self::MainThread(simulation) => simulation.update(),
            Self::Threaded(thread) => {
                if let Some(error) = thread.take_error() {
                    return Err(error.into());
                }

                Ok(())
            },
        }
    }

    pub(crate) fn suspend(&mut self) {
        match self {
            Self::MainThread(simulation) => simulation.suspend(),
            Self::Threaded(thread) => thread.pause(),
        }
    }

    pub(crate) fn resume(&mut self) {
        match self {
            Self::MainThread(simulation) => simulation.resume(),
            Self::Threaded(thread) => thread.resume(),
        }
    }

    pub(crate) fn stop(self) -> Option<RuntimeError> {
        let Self::Threaded(thread) = self else {
            return None;
        };

        match thread.stop() {
            Ok((_, Some(error))) => Some(error.into()),
            Ok((_, None)) => None,
            Err(error) => Some(error.into()),
        }
    }

    #[cfg(not(target_family = "wasm"))]
    fn automatic(task: SimulationTask, tick_rate: u32) -> Result<Self, RuntimeError> {
        Self::threaded(task, tick_rate)
    }

    #[cfg(target_family = "wasm")]
    fn automatic(task: SimulationTask, tick_rate: u32) -> Result<Self, RuntimeError> {
        Ok(Self::MainThread(Box::new(LocalSimulation::start(task, tick_rate)?)))
    }

    #[cfg(not(target_family = "wasm"))]
    fn threaded(task: SimulationTask, tick_rate: u32) -> Result<Self, RuntimeError> {
        Ok(Self::Threaded(Thread::spawn("luzure-simulation", task, tick_rate)?))
    }

    #[cfg(target_family = "wasm")]
    fn threaded(_task: SimulationTask, _tick_rate: u32) -> Result<Self, RuntimeError> {
        Err(luzure_thread::ThreadError::UnsupportedPlatform.into())
    }
}
