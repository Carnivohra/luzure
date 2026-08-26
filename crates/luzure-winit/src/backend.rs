mod application;
mod handle;

use application::WinitApplication;
use handle::WinitBackendHandle;

use luzure_backend::{Backend, backend::{BackendApplication, BackendError}};
use winit::{event_loop::EventLoop};

pub struct WinitBackend;

impl WinitBackend {
    pub const fn new() -> Self {
        Self
    }
}

impl Backend for WinitBackend {
    fn run<A: BackendApplication>(self, application: A) -> Result<(), A::Error> {
        let event_loop = EventLoop::new()
            .map_err(|_| BackendError::EventLoopInitialization)?;

        let mut application = WinitApplication::new(application);

        event_loop.run_app(&mut application)
            .map_err(|_| BackendError::EventLoop)?;

        if let Some(error) = application.take_error() {
            return Err(error);
        }

        Ok(())
    }
}
