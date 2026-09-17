mod error;
mod task;

#[cfg(not(target_family = "wasm"))]
mod state;

pub use error::ThreadError;
pub use task::ThreadTask;

#[cfg(not(target_family = "wasm"))]
use state::ThreadState;

use std::time::Duration;

pub(crate) const MAX_CATCH_UP_TICKS: u32 = 4;

#[cfg(not(target_family = "wasm"))]
use std::{sync::{Arc, atomic::Ordering}, thread::{self, JoinHandle}, time::Instant};

#[cfg(not(target_family = "wasm"))]
pub struct Thread<T: ThreadTask> {
    state: Arc<ThreadState<T::Error>>,
    handle: Option<JoinHandle<T>>,
}

#[cfg(not(target_family = "wasm"))]
impl<T: ThreadTask> Thread<T> {
    pub fn spawn(name: &str, task: T, tick_rate: u32) -> Result<Self, ThreadError> {
        tick_interval(tick_rate)?;

        let state = Arc::new(ThreadState::new(tick_rate));
        let thread_state = Arc::clone(&state);

        let handle = thread::Builder::new()
            .name(name.to_owned())
            .spawn(move || Self::run(task, thread_state))
            .map_err(ThreadError::Spawn)?;

        Ok(Self {
            state,
            handle: Some(handle),
        })
    }

    pub fn tick_rate(&self) -> u32 {
        self.state.tick_rate.load(Ordering::Acquire)
    }

    pub fn set_tick_rate(&self, tick_rate: u32) -> Result<(), ThreadError> {
        tick_interval(tick_rate)?;

        if self.state.tick_rate.swap(tick_rate, Ordering::AcqRel) != tick_rate {
            self.wake();
        }

        Ok(())
    }

    pub fn pause(&self) {
        if !self.state.paused.swap(true, Ordering::AcqRel) {
            self.wake();
        }
    }

    pub fn resume(&self) {
        if self.state.paused.swap(false, Ordering::AcqRel) {
            self.wake();
        }
    }

    pub fn take_error(&mut self) -> Option<T::Error> {
        self.state.error.take()
    }

    pub fn stop(mut self) -> Result<(T, Option<T::Error>), ThreadError> {
        self.signal_stop();

        let task = self.handle.take()
            .expect("running thread must have a join handle")
            .join()
            .map_err(|_| ThreadError::Panic)?;

        Ok((task, self.state.error.take()))
    }

    fn run(mut task: T, state: Arc<ThreadState<T::Error>>) -> T {
        if let Err(task_error) = task.start() {
            state.error.store(task_error);
            state.running.store(false, Ordering::Release);
            return task;
        }

        let mut current_tick_rate = state.tick_rate.load(Ordering::Acquire);
        let mut tick_interval = Self::tick_interval(current_tick_rate);
        let mut next_tick = Instant::now();
        let mut catch_up_ticks = 0;

        while state.running.load(Ordering::Acquire) {
            if state.paused.load(Ordering::Acquire) {
                thread::park();
                next_tick = Instant::now();
                catch_up_ticks = 0;
                continue;
            }

            let updated_tick_rate = state.tick_rate.load(Ordering::Acquire);

            if updated_tick_rate != current_tick_rate {
                current_tick_rate = updated_tick_rate;
                tick_interval = Self::tick_interval(current_tick_rate);
                next_tick = Instant::now() + tick_interval;
                catch_up_ticks = 0;
            }

            let now = Instant::now();

            if now < next_tick {
                thread::park_timeout(next_tick - now);
                continue;
            }

            if let Err(task_error) = task.tick(tick_interval) {
                state.error.store(task_error);
                state.running.store(false, Ordering::Release);
                break;
            }

            next_tick = advance_tick(next_tick, now, Instant::now(), tick_interval, &mut catch_up_ticks);
        }

        task
    }

    fn tick_interval(tick_rate: u32) -> Duration {
        Duration::from_secs_f64(1.0 / f64::from(tick_rate))
    }

    fn signal_stop(&self) {
        self.state.running.store(false, Ordering::Release);
        self.wake();
    }

    fn wake(&self) {
        if let Some(handle) = &self.handle {
            handle.thread().unpark();
        }
    }
}

#[cfg(not(target_family = "wasm"))]
fn advance_tick(deadline: Instant, started: Instant, now: Instant, interval: Duration, catch_up_ticks: &mut u32) -> Instant {
    let next = deadline + interval;

    if next > now {
        *catch_up_ticks = 0;
        return next;
    }

    *catch_up_ticks += 1;

    if *catch_up_ticks >= MAX_CATCH_UP_TICKS {
        *catch_up_ticks = 0;
        return started + interval;
    }

    next
}

pub(crate) fn tick_interval(tick_rate: u32) -> Result<Duration, ThreadError> {
    if tick_rate == 0 {
        return Err(ThreadError::InvalidTickRate);
    }

    let interval = Duration::from_secs_f64(1.0 / f64::from(tick_rate));

    if interval.is_zero() {
        return Err(ThreadError::InvalidTickRate);
    }

    Ok(interval)
}

#[cfg(not(target_family = "wasm"))]
impl<T: ThreadTask> Drop for Thread<T> {
    fn drop(&mut self) {
        self.signal_stop();

        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
