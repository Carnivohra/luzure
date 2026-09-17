use luzure_backend::{Window, backend::{BackendError, BackendHandle}, window::{WindowDescriptor, WindowId}};
use winit::{dpi::PhysicalSize, event_loop::ActiveEventLoop, window::WindowAttributes};

#[cfg(target_family = "wasm")]
use winit::platform::web::WindowAttributesExtWebSys;

use crate::window::{WinitWindow, WinitWindowRegistry};

use std::rc::Rc;

pub(super) struct WinitBackendHandle<'a> {
    event_loop: &'a ActiveEventLoop,
    windows: &'a mut WinitWindowRegistry,
}

impl<'a> WinitBackendHandle<'a> {
    pub(super) fn new(event_loop: &'a ActiveEventLoop, windows: &'a mut WinitWindowRegistry) -> Self {
        Self {
            event_loop,
            windows,
        }
    }
}

impl BackendHandle for WinitBackendHandle<'_> {
    fn create_window(&mut self, descriptor: WindowDescriptor) -> Result<Window, BackendError> {
        let attributes = WindowAttributes::default()
            .with_title(descriptor.title)
            .with_inner_size(PhysicalSize::new(descriptor.width, descriptor.height))
            .with_resizable(descriptor.resizable)
            .with_visible(descriptor.visible);

        #[cfg(target_family = "wasm")]
        let attributes = attributes.with_append(true);

        let window = self.event_loop.create_window(attributes)
            .map_err(|_| BackendError::WindowCreation)?;

        let winit_window = Rc::new(WinitWindow::new(window));
        let window_id = self.windows.insert(Rc::clone(&winit_window))?;

        Ok(Window::new(window_id, winit_window))
    }

    fn destroy_window(&mut self, window_id: WindowId) -> Result<(), BackendError> {
        self.windows.remove(window_id)
    }

    fn exit(&mut self) -> Result<(), BackendError> {
        self.event_loop.exit();

        Ok(())
    }
}
