use crate::{ThreadTask, runner::local::LocalTask};

#[cfg(not(target_family = "wasm"))]
use crate::Thread;

#[cfg(target_family = "wasm")]
type LocalTaskStorage<T> = LocalTask<T>;

#[cfg(not(target_family = "wasm"))]
type LocalTaskStorage<T> = Box<LocalTask<T>>;

pub(super) enum TaskRunnerState<T: ThreadTask> {
    MainThread(LocalTaskStorage<T>),

    #[cfg(not(target_family = "wasm"))]
    Threaded(Thread<T>),
}

impl<T: ThreadTask> TaskRunnerState<T> {
    pub(super) fn main_thread(task: LocalTask<T>) -> Self {
        #[cfg(target_family = "wasm")]
        {
            Self::MainThread(task)
        }

        #[cfg(not(target_family = "wasm"))]
        {
            Self::MainThread(Box::new(task))
        }
    }
}
