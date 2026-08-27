use luzure_backend::window::{WindowId, WindowSource};
use winit::window::WindowId as WinitWindowId;

use std::sync::Arc;

use crate::window::WinitWindow;

pub(crate) struct WinitWindowEntry {
    window_id: WindowId,
    window: Arc<WinitWindow>,
}

impl WinitWindowEntry {
    pub(crate) const fn new(window_id: WindowId, window: Arc<WinitWindow>) -> Self {
        Self {
            window_id,
            window,
        }
    }

    pub(crate) fn winit_id(&self) -> WinitWindowId {
        self.window.id()
    }

    pub(crate) const fn window_id(&self) -> WindowId {
        self.window_id
    }

    pub(crate) fn request_redraw(&self) {
        self.window.request_redraw();
    }
}
