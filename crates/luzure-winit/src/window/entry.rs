use luzure_backend::window::WindowId;
use winit::window::WindowId as WinitWindowId;

pub(crate) struct WinitWindowEntry {
    pub winit_id: WinitWindowId,
    pub window_id: WindowId,
}

impl WinitWindowEntry {
    pub(crate) const fn new(winit_id: WinitWindowId, window_id: WindowId) -> Self {
        Self {
            winit_id,
            window_id,
        }
    }

    pub(crate) const fn winit_id(&self) -> WinitWindowId {
        self.winit_id
    }

    pub(crate) const fn window_id(&self) -> WindowId {
        self.window_id
    }
}
