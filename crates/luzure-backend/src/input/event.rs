use luzure_input::{keyboard::KeyboardKey, mouse::MouseButton};

use crate::window::WindowId;

pub enum InputEvent {
    CursorEntered { window_id: WindowId },
    CursorLeft { window_id: WindowId },
    CursorMoved { window_id: WindowId, x: f64, y: f64 },
    KeyboardKeyPressed { window_id: WindowId, key: KeyboardKey },
    KeyboardKeyReleased { window_id: WindowId, key: KeyboardKey },
    MouseButtonPressed { window_id: WindowId, button: MouseButton },
    MouseButtonReleased { window_id: WindowId, button: MouseButton },
    MouseMotion { x: f64, y: f64 },
    MouseWheelLines { window_id: WindowId, x: f32, y: f32 },
    MouseWheelPixels { window_id: WindowId, x: f64, y: f64 },
}
