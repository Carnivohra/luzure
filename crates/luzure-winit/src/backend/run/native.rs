use luzure_backend::backend::{BackendApplication, BackendError};
use winit::event_loop::EventLoop;

use crate::backend::application::WinitApplication;

pub(super) fn run<A: BackendApplication + 'static>(event_loop: EventLoop<()>, application: A) -> Result<(), A::Error> {
    let mut application = WinitApplication::new(application);
    let result = event_loop.run_app(&mut application);

    if let Some(error) = application.take_error() {
        return Err(error);
    }

    result.map_err(|_| BackendError::EventLoop.into())
}
