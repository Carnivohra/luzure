mod mode;
mod runner;
pub mod thread;

pub use mode::ThreadMode;
pub use runner::TaskRunner;
pub use thread::{ThreadError, ThreadTask};

#[cfg(not(target_family = "wasm"))]
pub use thread::Thread;
