use luzure_backend::{backend::BackendApplication};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::WindowId as WinitWindowId};

use crate::backend::WinitBackendHandle;

pub(super) struct WinitApplication<A: BackendApplication> {
    application: A,
    windows: Vec<Option<WinitWindowId>>,
    started: bool,
    error: Option<A::Error>,
}

impl<A: BackendApplication> WinitApplication<A> {
    pub(super) fn new(application: A) -> Self {
        Self {
            application,
            windows: vec![],
            started: false,
            error: None,
        }
    }

    pub(super) fn take_error(&mut self) -> Option<A::Error> {
        self.error.take()
    }
}

impl<A: BackendApplication> ApplicationHandler for WinitApplication<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut handle = WinitBackendHandle::new(event_loop, &mut self.windows);

        if !self.started {
            if let Err(error) = self.application.started(&mut handle) {
                self.error = Some(error);
                return event_loop.exit();
            }

            self.started = true;
        }

        if let Err(error) = self.application.resumed(&mut handle) {
            self.error = Some(error);
            event_loop.exit();
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        let mut handle = WinitBackendHandle::new(event_loop, &mut self.windows);

        if let Err(error) = self.application.suspended(&mut handle) {
            self.error = Some(error);
            event_loop.exit();
        }
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _window_id: WinitWindowId, _event: WindowEvent) {
        todo!()
    }
}
