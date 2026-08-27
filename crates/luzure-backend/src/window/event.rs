mod kind;

pub use kind::WindowEventKind;

use crate::window::WindowId;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowEvent {
    pub window_id: WindowId,
    pub kind: WindowEventKind,
}
