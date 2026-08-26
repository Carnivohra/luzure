use crate::window::WindowId;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowEvent {
    CloseRequested { window_id: WindowId },
    RedrawRequested { window_id: WindowId },
    Resized { window_id: WindowId, width: u32, height: u32 },
    Focused { window_id: WindowId, focuses: bool },
}
