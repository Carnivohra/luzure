use luzure_backend::backend::{BackendApplication, BackendError};
use winit::event_loop::EventLoop;

pub(in crate::backend) fn run<A: BackendApplication + 'static>(application: A) -> Result<(), A::Error> {
    let event_loop = EventLoop::new()
        .map_err(|_| BackendError::EventLoopInitialization)?;

    super::native::run(event_loop, application)
}
