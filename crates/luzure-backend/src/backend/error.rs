use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("failed to initialize event loop")]
    EventLoopInitialization,

    #[error("event loop failed")]
    EventLoop,

    #[error("failed to create window")]
    WindowCreation,

    #[error("window does not exist")]
    InvalidWindow,
}
