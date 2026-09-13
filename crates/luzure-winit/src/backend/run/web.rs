use luzure_backend::backend::{BackendApplication, BackendError};
use winit::{event_loop::EventLoop, platform::web::EventLoopExtWebSys};

use crate::backend::application::WinitApplication;

pub(in crate::backend) fn run<A: BackendApplication + 'static>(application: A) -> Result<(), A::Error> {
    let event_loop = EventLoop::new()
        .map_err(|_| BackendError::EventLoopInitialization)?;

    let application = WinitApplication::new(application);

    event_loop.spawn_app(application);

    Ok(())
}
