use luzure_backend::{input::InputEvent, window::WindowId};
use winit::{event::{DeviceEvent, ElementState, MouseScrollDelta, WindowEvent}, keyboard::PhysicalKey};

use crate::input::{keyboard::keyboard_key, mouse::mouse_button};

pub(crate) fn window_event(window_id: WindowId, event: &WindowEvent) -> Option<InputEvent> {
    Some(match event {
        WindowEvent::CursorEntered { .. } => InputEvent::CursorEntered { window_id },
        WindowEvent::CursorLeft { .. } => InputEvent::CursorLeft { window_id },
        WindowEvent::CursorMoved { position, .. } => InputEvent::CursorMoved {
            window_id,
            x: position.x,
            y: position.y,
        },
        WindowEvent::KeyboardInput { event, .. } => {
            let PhysicalKey::Code(code) = event.physical_key else { return None };
            let key = keyboard_key(code)?;

            match event.state {
                ElementState::Pressed => InputEvent::KeyboardKeyPressed { window_id, key },
                ElementState::Released => InputEvent::KeyboardKeyReleased { window_id, key },
            }
        }
        WindowEvent::MouseInput { state, button, .. } => {
            let button = mouse_button(*button)?;

            match state {
                ElementState::Pressed => InputEvent::MouseButtonPressed { window_id, button },
                ElementState::Released => InputEvent::MouseButtonReleased { window_id, button },
            }
        }
        WindowEvent::MouseWheel { delta, .. } => match delta {
            MouseScrollDelta::LineDelta(x, y) => InputEvent::MouseWheelLines {
                window_id,
                x: *x,
                y: *y,
            },
            MouseScrollDelta::PixelDelta(position) => InputEvent::MouseWheelPixels {
                window_id,
                x: position.x,
                y: position.y,
            },
        },
        _ => return None,
    })
}

pub(crate) fn device_event(event: &DeviceEvent) -> Option<InputEvent> {
    match event {
        DeviceEvent::MouseMotion { delta: (x, y) } => Some(InputEvent::MouseMotion { x: *x, y: *y }),
        _ => None,
    }
}
