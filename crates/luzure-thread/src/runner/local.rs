use std::time::Duration;

#[cfg(not(target_family = "wasm"))]
use std::time::Instant;

#[cfg(target_family = "wasm")]
use web_time::Instant;

use crate::{ThreadError, ThreadTask, thread::{MAX_CATCH_UP_TICKS, tick_interval}};

pub(super) struct LocalTask<T: ThreadTask> {
    accumulator: Duration,
    error: Option<T::Error>,
    last_update: Instant,
    paused: bool,
    running: bool,
    task: T,
    tick_interval: Duration,
    tick_rate: u32,
}

impl<T: ThreadTask> LocalTask<T> {
    pub(super) fn start(mut task: T, tick_rate: u32) -> Result<Self, ThreadError> {
        let tick_interval = tick_interval(tick_rate)?;
        let error = task.start().err();
        let running = error.is_none();

        Ok(Self {
            accumulator: Duration::ZERO,
            error,
            last_update: Instant::now(),
            paused: false,
            running,
            task,
            tick_interval,
            tick_rate,
        })
    }

    pub(super) fn update(&mut self) -> Result<(), T::Error> {
        if let Some(error) = self.error.take() {
            return Err(error);
        }

        if self.paused || !self.running {
            return Ok(());
        }

        let now = Instant::now();
        let elapsed = now.saturating_duration_since(self.last_update);
        let maximum_elapsed = self.tick_interval.saturating_mul(MAX_CATCH_UP_TICKS);

        self.last_update = now;
        self.accumulator = self.accumulator.saturating_add(elapsed.min(maximum_elapsed));

        while self.accumulator >= self.tick_interval {
            if let Err(error) = self.task.tick(self.tick_interval) {
                self.running = false;
                return Err(error);
            }

            self.accumulator -= self.tick_interval;
        }

        Ok(())
    }

    pub(super) const fn tick_rate(&self) -> u32 {
        self.tick_rate
    }

    pub(super) fn set_tick_rate(&mut self, tick_rate: u32) -> Result<(), ThreadError> {
        if self.tick_rate == tick_rate {
            return Ok(());
        }

        self.tick_interval = tick_interval(tick_rate)?;
        self.tick_rate = tick_rate;
        self.accumulator = Duration::ZERO;
        self.last_update = Instant::now();

        Ok(())
    }

    pub(super) const fn pause(&mut self) {
        self.paused = true;
    }

    pub(super) fn resume(&mut self) {
        if !self.paused {
            return;
        }

        self.accumulator = Duration::ZERO;
        self.last_update = Instant::now();
        self.paused = false;
    }

    pub(super) fn stop(self) -> (T, Option<T::Error>) {
        (self.task, self.error)
    }
}
