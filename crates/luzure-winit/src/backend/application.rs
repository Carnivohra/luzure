use luzure_backend::{backend::BackendApplication, window::{WindowEvent, WindowEventKind}};
use winit::{application::ApplicationHandler, event::{DeviceEvent, DeviceId, WindowEvent as WinitWindowEvent}, event_loop::ActiveEventLoop, window::WindowId as WinitWindowId};

use crate::{backend::WinitBackendHandle, input, window::WinitWindowRegistry};

pub(super) struct WinitApplication<A: BackendApplication> {
    application: A,
    windows: WinitWindowRegistry,
    started: bool,
    resumed: bool,
    error: Option<A::Error>,
}

impl<A: BackendApplication> WinitApplication<A> {
    pub(super) fn new(application: A) -> Self {
        Self {
            application,
            windows: WinitWindowRegistry::new(),
            started: false,
            resumed: false,
            error: None,
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub(super) fn take_error(&mut self) -> Option<A::Error> {
        self.error.take()
    }
}

impl<A: BackendApplication> ApplicationHandler for WinitApplication<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.resumed || self.error.is_some() || event_loop.exiting() {
            return;
        }

        let mut handle = WinitBackendHandle::new(event_loop, &mut self.windows);

        if !self.started {
            if let Err(error) = self.application.started(&mut handle) {
                self.error = Some(error);
                return event_loop.exit();
            }

            self.started = true;
        }

        if event_loop.exiting() {
            return;
        }

        if let Err(error) = self.application.resumed(&mut handle) {
            self.error = Some(error);
            return event_loop.exit();
        }

        self.resumed = true;
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        if !self.resumed {
            return;
        }

        self.resumed = false;
        let mut handle = WinitBackendHandle::new(event_loop, &mut self.windows);

        if let Err(error) = self.application.suspended(&mut handle) {
            if self.error.is_none() {
                self.error = Some(error);
            }

            event_loop.exit();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, winit_window_id: WinitWindowId, winit_event: WinitWindowEvent) {
        if self.error.is_some() || event_loop.exiting() {
            return;
        }

        let Some(window_id) = self.windows.window_id(winit_window_id) else { return };

        if let Some(event) = input::window_event(window_id, &winit_event) {
            if self.resumed {
                self.application.input_event(event);
            }

            return;
        }

        let kind = match winit_event {
            WinitWindowEvent::CloseRequested => WindowEventKind::CloseRequested,
            WinitWindowEvent::RedrawRequested => WindowEventKind::RedrawRequested,
            WinitWindowEvent::Resized(size) => WindowEventKind::Resized { width: size.width, height: size.height },
            WinitWindowEvent::Focused(focused) => WindowEventKind::Focused { focused },
            WinitWindowEvent::Occluded(occluded) => WindowEventKind::Occluded { occluded },
            _ => return
        };

        let event = WindowEvent { window_id, kind };

        let mut handle = WinitBackendHandle::new(event_loop, &mut self.windows);

        if let Err(error) = self.application.window_event(&mut handle, event) {
            self.error = Some(error);
            event_loop.exit();
        }
    }

    fn device_event(&mut self, event_loop: &ActiveEventLoop, _device_id: DeviceId, winit_event: DeviceEvent) {
        if !self.resumed || self.error.is_some() || event_loop.exiting() {
            return;
        }

        if let Some(event) = input::device_event(&winit_event) {
            self.application.input_event(event);
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if !self.resumed || self.error.is_some() || event_loop.exiting() {
            return;
        }

        let mut handle = WinitBackendHandle::new(event_loop, &mut self.windows);

        if let Err(error) = self.application.update(&mut handle) {
            self.error = Some(error);
            return event_loop.exit();
        }
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        self.resumed = false;

        if self.started {
            self.started = false;
            let mut handle = WinitBackendHandle::new(event_loop, &mut self.windows);

            if let Err(error) = self.application.stopped(&mut handle) {
                if self.error.is_none() {
                    self.error = Some(error);
                }
            }
        }

        self.windows.clear();

        #[cfg(target_family = "wasm")]
        if let Some(error) = self.error.take() {
            web_sys::console::error_1(&error.to_string().into());
        }
    }
}
