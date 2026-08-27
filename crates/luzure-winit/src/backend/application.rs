use luzure_backend::{backend::BackendApplication, window::{WindowEvent, WindowEventKind}};
use winit::{application::ApplicationHandler, event::WindowEvent as WinitWindowEvent, event_loop::ActiveEventLoop, window::WindowId as WinitWindowId};

use crate::{backend::WinitBackendHandle, window::WinitWindowEntry};

pub(super) struct WinitApplication<A: BackendApplication> {
    application: A,
    windows: Vec<Option<WinitWindowEntry>>,
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

    fn window_event(&mut self, event_loop: &ActiveEventLoop, winit_window_id: WinitWindowId, winit_event: WinitWindowEvent) {
        let Some(window_id) = self.windows.iter()
            .flatten()
            .find(|window| window.winit_id() == winit_window_id)
            .map(WinitWindowEntry::window_id) else { return };

        let kind = match winit_event {
            WinitWindowEvent::CloseRequested => WindowEventKind::CloseRequested,
            WinitWindowEvent::RedrawRequested => WindowEventKind::RedrawRequested,
            WinitWindowEvent::Resized(size) => WindowEventKind::Resized { width: size.width, height: size.height },
            WinitWindowEvent::Focused(focused) => WindowEventKind::Focused { focused },
            _ => return
        };

        let event = WindowEvent { window_id, kind };

        let mut handle = WinitBackendHandle::new(event_loop, &mut self.windows);

        if let Err(error) = self.application.window_event(&mut handle, event) {
            self.error = Some(error);
            event_loop.exit();
        }
    }
}
