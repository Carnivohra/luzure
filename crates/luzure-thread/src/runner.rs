mod local;
mod state;

use local::LocalTask;
use state::TaskRunnerState;

use crate::{ThreadError, ThreadMode, ThreadTask};

#[cfg(not(target_family = "wasm"))]
use crate::Thread;

pub struct TaskRunner<T: ThreadTask> {
    state: TaskRunnerState<T>,
}

impl<T: ThreadTask> TaskRunner<T> {
    pub fn start(name: &str, task: T, mode: ThreadMode, tick_rate: u32) -> Result<Self, ThreadError> {
        let state = match mode {
            ThreadMode::Automatic => Self::automatic(name, task, tick_rate)?,
            ThreadMode::MainThread => TaskRunnerState::main_thread(LocalTask::start(task, tick_rate)?),
            ThreadMode::Threaded => Self::threaded(name, task, tick_rate)?,
        };

        Ok(Self { state })
    }

    pub fn update(&mut self) -> Result<(), T::Error> {
        match &mut self.state {
            TaskRunnerState::MainThread(task) => task.update(),

            #[cfg(not(target_family = "wasm"))]
            TaskRunnerState::Threaded(thread) => match thread.take_error() {
                Some(error) => Err(error),
                None => Ok(()),
            },
        }
    }

    pub fn tick_rate(&self) -> u32 {
        match &self.state {
            TaskRunnerState::MainThread(task) => task.tick_rate(),

            #[cfg(not(target_family = "wasm"))]
            TaskRunnerState::Threaded(thread) => thread.tick_rate(),
        }
    }

    pub fn main_thread_task(&self) -> Option<&T> {
        match &self.state {
            TaskRunnerState::MainThread(task) => Some(task.task()),

            #[cfg(not(target_family = "wasm"))]
            TaskRunnerState::Threaded(_) => None,
        }
    }

    pub fn set_tick_rate(&mut self, tick_rate: u32) -> Result<(), ThreadError> {
        match &mut self.state {
            TaskRunnerState::MainThread(task) => task.set_tick_rate(tick_rate),

            #[cfg(not(target_family = "wasm"))]
            TaskRunnerState::Threaded(thread) => thread.set_tick_rate(tick_rate),
        }
    }

    pub fn pause(&mut self) {
        match &mut self.state {
            TaskRunnerState::MainThread(task) => task.pause(),

            #[cfg(not(target_family = "wasm"))]
            TaskRunnerState::Threaded(thread) => thread.pause(),
        }
    }

    pub fn resume(&mut self) {
        match &mut self.state {
            TaskRunnerState::MainThread(task) => task.resume(),

            #[cfg(not(target_family = "wasm"))]
            TaskRunnerState::Threaded(thread) => thread.resume(),
        }
    }

    pub fn stop(self) -> Result<(T, Option<T::Error>), ThreadError> {
        match self.state {
            TaskRunnerState::MainThread(task) => Ok(task.stop()),

            #[cfg(not(target_family = "wasm"))]
            TaskRunnerState::Threaded(thread) => thread.stop(),
        }
    }

    #[cfg(not(target_family = "wasm"))]
    fn automatic(name: &str, task: T, tick_rate: u32) -> Result<TaskRunnerState<T>, ThreadError> {
        Self::threaded(name, task, tick_rate)
    }

    #[cfg(target_family = "wasm")]
    fn automatic(_name: &str, task: T, tick_rate: u32) -> Result<TaskRunnerState<T>, ThreadError> {
        Ok(TaskRunnerState::main_thread(LocalTask::start(task, tick_rate)?))
    }

    #[cfg(not(target_family = "wasm"))]
    fn threaded(name: &str, task: T, tick_rate: u32) -> Result<TaskRunnerState<T>, ThreadError> {
        Ok(TaskRunnerState::Threaded(Thread::spawn(name, task, tick_rate)?))
    }

    #[cfg(target_family = "wasm")]
    fn threaded(_name: &str, _task: T, _tick_rate: u32) -> Result<TaskRunnerState<T>, ThreadError> {
        Err(ThreadError::UnsupportedPlatform)
    }
}
