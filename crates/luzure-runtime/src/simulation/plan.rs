use luzure_thread::{ThreadError, ThreadMode};

use super::Simulation;

pub(crate) struct SimulationPlan {
    thread_mode: ThreadMode,
    tick_rate: u32,
}

impl SimulationPlan {
    pub(crate) const fn new() -> Self {
        Self {
            thread_mode: ThreadMode::Automatic,
            tick_rate: Simulation::DEFAULT_TICK_RATE,
        }
    }

    pub(crate) const fn thread_mode(&self) -> ThreadMode {
        self.thread_mode
    }

    pub(crate) const fn set_thread_mode(&mut self, thread_mode: ThreadMode) {
        self.thread_mode = thread_mode;
    }

    pub(crate) const fn tick_rate(&self) -> u32 {
        self.tick_rate
    }

    pub(crate) fn set_tick_rate(&mut self, tick_rate: u32) -> Result<(), ThreadError> {
        if tick_rate == 0 {
            return Err(ThreadError::InvalidTickRate);
        }

        self.tick_rate = tick_rate;

        Ok(())
    }
}
