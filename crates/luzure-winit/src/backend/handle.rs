use luzure_backend::{Window, backend::{BackendError, BackendHandle}, window::{WindowDescriptor, WindowId}};
use winit::{dpi::PhysicalSize, event_loop::{ActiveEventLoop}, window::{WindowId as WinitWindowId, WindowAttributes}};

use crate::window::WinitWindow;

pub(super) struct WinitBackendHandle<'a> {
    event_loop: &'a ActiveEventLoop,
    windows: &'a mut Vec<Option<WinitWindowId>>,
}

impl<'a> WinitBackendHandle<'a> {
    pub(super) fn new(event_loop: &'a ActiveEventLoop, windows: &'a mut Vec<Option<WinitWindowId>>) -> Self {
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

        let window = self.event_loop.create_window(attributes)
            .map_err(|_| BackendError::WindowCreation)?;

        let window_id = WindowId::new(self.windows.len() as u64);
        self.windows.push(Some(window.id()));
        let winit_window = WinitWindow::new(window);
        Ok(Window::new(window_id, Box::new(winit_window)))
    }

    fn destroy_window(&mut self, _window: Window) -> Result<(), BackendError> {
        todo!()
    }

    fn exit(&mut self) -> Result<(), BackendError> {
        self.event_loop.exit();
        Ok(())
    }
}
