use luzure_backend::{Window, backend::{BackendError, BackendHandle}, window::{WindowDescriptor, WindowId}};
use winit::{dpi::PhysicalSize, event_loop::{ActiveEventLoop}, window::WindowAttributes};

use crate::window::{WinitWindow, WinitWindowEntry};

use std::sync::Arc;

pub(super) struct WinitBackendHandle<'a> {
    event_loop: &'a ActiveEventLoop,
    windows: &'a mut Vec<Option<WinitWindowEntry>>,
}

impl<'a> WinitBackendHandle<'a> {
    pub(super) fn new(event_loop: &'a ActiveEventLoop, windows: &'a mut Vec<Option<WinitWindowEntry>>) -> Self {
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

        let winit_window = Arc::new(WinitWindow::new(window));
        let window_id = WindowId::new(self.windows.len() as u64);

        self.windows.push(Some(WinitWindowEntry::new(window_id, Arc::clone(&winit_window))));

        Ok(Window::new(window_id, winit_window))
    }

    fn destroy_window(&mut self, window_id: WindowId) -> Result<(), BackendError> {
        let index = usize::try_from(window_id.value())
            .map_err(|_| BackendError::InvalidWindow)?;

        let window = self.windows.get_mut(index)
            .ok_or(BackendError::InvalidWindow)?;

        if window.as_ref().is_none_or(|window| window.window_id() != window_id) {
            return Err(BackendError::InvalidWindow);
        }

        window.take();

        Ok(())
    }

    fn exit(&mut self) -> Result<(), BackendError> {
        self.event_loop.exit();
        
        Ok(())
    }
}
