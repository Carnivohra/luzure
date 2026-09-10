mod error;
mod task;

pub use error::ThreadError;
pub use task::ThreadTask;

use error::ThreadErrorSlot;

use std::{num::NonZeroU32, sync::{Arc, atomic::{AtomicBool, AtomicU32, Ordering}}, thread::{self, JoinHandle}, time::{Duration, Instant}};

pub struct Thread<T: ThreadTask> {
    error: Arc<ThreadErrorSlot<T::Error>>,
    running: Arc<AtomicBool>,
    tick_rate: Arc<AtomicU32>,
    handle: Option<JoinHandle<T>>,
}

impl<T: ThreadTask> Thread<T> {
    pub fn spawn(name: &str, task: T, tick_rate: u32) -> Result<Self, ThreadError> {
        let tick_rate = NonZeroU32::new(tick_rate)
            .ok_or(ThreadError::InvalidTickRate)?;

        let running = Arc::new(AtomicBool::new(true));
        let tick_rate = Arc::new(AtomicU32::new(tick_rate.get()));
        let error = Arc::new(ThreadErrorSlot::new());
        let thread_error = Arc::clone(&error);
        let thread_running = Arc::clone(&running);
        let thread_tick_rate = Arc::clone(&tick_rate);

        let handle = thread::Builder::new()
            .name(name.to_owned())
            .spawn(move || Self::run(task, thread_tick_rate, thread_running, thread_error))
            .map_err(ThreadError::Spawn)?;

        Ok(Self {
            error,
            running,
            tick_rate,
            handle: Some(handle),
        })
    }

    pub fn tick_rate(&self) -> u32 {
        self.tick_rate.load(Ordering::Acquire)
    }

    pub fn set_tick_rate(&self, tick_rate: u32) -> Result<(), ThreadError> {
        let tick_rate = NonZeroU32::new(tick_rate)
            .ok_or(ThreadError::InvalidTickRate)?;

        self.tick_rate.store(tick_rate.get(), Ordering::Release);
        self.wake();

        Ok(())
    }

    pub fn take_error(&mut self) -> Option<T::Error> {
        self.error.take()
    }

    pub fn stop(mut self) -> Result<(T, Option<T::Error>), ThreadError> {
        self.signal_stop();

        let task = self.handle.take()
            .expect("running thread must have a join handle")
            .join()
            .map_err(|_| ThreadError::Panic)?;

        Ok((task, self.error.take()))
    }

    fn run(mut task: T, tick_rate: Arc<AtomicU32>, running: Arc<AtomicBool>, error: Arc<ThreadErrorSlot<T::Error>>) -> T {
        if let Err(task_error) = task.start() {
            error.store(task_error);
            running.store(false, Ordering::Release);
            return task;
        }

        let mut current_tick_rate = tick_rate.load(Ordering::Acquire);
        let mut tick_interval = Self::tick_interval(current_tick_rate);
        let mut next_tick = Instant::now();

        while running.load(Ordering::Acquire) {
            let updated_tick_rate = tick_rate.load(Ordering::Acquire);

            if updated_tick_rate != current_tick_rate {
                current_tick_rate = updated_tick_rate;
                tick_interval = Self::tick_interval(current_tick_rate);
                next_tick = Instant::now() + tick_interval;
            }

            let now = Instant::now();

            if now < next_tick {
                thread::park_timeout(next_tick - now);
                continue;
            }

            if let Err(task_error) = task.tick(tick_interval) {
                error.store(task_error);
                running.store(false, Ordering::Release);
                break;
            }

            next_tick += tick_interval;

            let now = Instant::now();

            if next_tick < now {
                next_tick = now + tick_interval;
            }
        }

        task
    }

    fn tick_interval(tick_rate: u32) -> Duration {
        Duration::from_secs_f64(1.0 / f64::from(tick_rate))
    }

    fn signal_stop(&self) {
        self.running.store(false, Ordering::Release);
        self.wake();
    }

    fn wake(&self) {
        if let Some(handle) = &self.handle {
            handle.thread().unpark();
        }
    }
}

impl<T: ThreadTask> Drop for Thread<T> {
    fn drop(&mut self) {
        self.signal_stop();

        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
