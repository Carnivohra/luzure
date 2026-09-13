mod slot;

pub(crate) use slot::ThreadErrorSlot;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ThreadError {
    #[error("thread tick rate must be greater than zero")]
    InvalidTickRate,

    #[error("threaded execution is not supported on this platform")]
    UnsupportedPlatform,

    #[error("failed to spawn thread")]
    Spawn(#[source] std::io::Error),

    #[error("thread panicked")]
    Panic,
}
