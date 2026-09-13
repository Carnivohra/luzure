use luzure_backend::window::WindowId;
use winit::window::WindowId as WinitWindowId;

use std::rc::Rc;

use crate::window::WinitWindow;

pub(crate) struct WinitWindowEntry {
    window_id: WindowId,
    window: Rc<WinitWindow>,
}

impl WinitWindowEntry {
    pub(crate) const fn new(window_id: WindowId, window: Rc<WinitWindow>) -> Self {
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
}
