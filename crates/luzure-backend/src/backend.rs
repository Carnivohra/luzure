mod application;
mod error;
mod handle;

pub use application::BackendApplication;
pub use error::BackendError;
pub use handle::BackendHandle;

pub trait Backend {
    fn run<A: BackendApplication>(self, application: A) -> Result<(), A::Error>;
}
