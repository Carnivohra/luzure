use luzure_backend::backend::{BackendApplication, BackendError};
use winit::event_loop::EventLoop;

use crate::backend::application::WinitApplication;

pub(in crate::backend) fn run<A: BackendApplication + 'static>(application: A) -> Result<(), A::Error> {
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
