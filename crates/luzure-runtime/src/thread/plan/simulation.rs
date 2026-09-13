use luzure_thread::ThreadError;

use crate::Simulation;

pub struct SimulationThreadPlan {
    tick_rate: u32,
}

impl SimulationThreadPlan {
    pub(crate) const fn new() -> Self {
        Self {
            tick_rate: Simulation::DEFAULT_TICK_RATE,
        }
    }

    pub const fn tick_rate(&self) -> u32 {
        self.tick_rate
    }

    pub fn set_tick_rate(&mut self, tick_rate: u32) -> Result<(), ThreadError> {
        if tick_rate == 0 {
            return Err(ThreadError::InvalidTickRate);
        }

        self.tick_rate = tick_rate;

        Ok(())
    }
}
