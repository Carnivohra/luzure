use luzure_thread::{ThreadError, ThreadTask};

use std::{num::NonZeroU32, time::{Duration, Instant}};

use crate::{runtime::RuntimeError, simulation::SimulationTask};

pub(in crate::thread) struct LocalSimulation {
    accumulator: Duration,
    last_update: Instant,
    paused: bool,
    task: SimulationTask,
    tick_interval: Duration,
}

impl LocalSimulation {
    const MAX_CATCH_UP_TICKS: u32 = 4;

    pub(in crate::thread) fn start(mut task: SimulationTask, tick_rate: u32) -> Result<Self, RuntimeError> {
        let tick_rate = NonZeroU32::new(tick_rate)
            .ok_or(ThreadError::InvalidTickRate)?;
        let tick_interval = Duration::from_secs_f64(1.0 / f64::from(tick_rate.get()));

        if tick_interval.is_zero() {
            return Err(ThreadError::InvalidTickRate.into());
        }

        task.start()?;

        Ok(Self {
            accumulator: Duration::ZERO,
            last_update: Instant::now(),
            paused: false,
            task,
            tick_interval,
        })
    }

    pub(in crate::thread) fn update(&mut self) -> Result<(), RuntimeError> {
        if self.paused {
            return Ok(());
        }

        let now = Instant::now();
        let elapsed = now.saturating_duration_since(self.last_update);
        let maximum_elapsed = self.tick_interval.saturating_mul(Self::MAX_CATCH_UP_TICKS);

        self.last_update = now;
        self.accumulator = self.accumulator.saturating_add(elapsed.min(maximum_elapsed));

        while self.accumulator >= self.tick_interval {
            self.task.tick(self.tick_interval)?;
            self.accumulator -= self.tick_interval;
        }

        Ok(())
    }

    pub(in crate::thread) const fn suspend(&mut self) {
        self.paused = true;
    }

    pub(in crate::thread) fn resume(&mut self) {
        if !self.paused {
            return;
        }

        self.accumulator = Duration::ZERO;
        self.last_update = Instant::now();
        self.paused = false;
    }
}
