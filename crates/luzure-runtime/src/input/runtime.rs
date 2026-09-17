use luzure_backend::{input::InputEvent, window::WindowId};
use luzure_input::input::InputState;

pub(crate) struct InputRuntime {
    focused_window: Option<WindowId>,
    cursor_window: Option<WindowId>,
    state: InputState,
}

impl InputRuntime {
    pub(crate) const fn new() -> Self {
        Self {
            focused_window: None,
            cursor_window: None,
            state: InputState::new(),
        }
    }

    pub(crate) fn update(&mut self, event: InputEvent) {
        match event {
            InputEvent::CursorEntered { window_id } => self.cursor_entered(window_id),
            InputEvent::CursorLeft { window_id } => {
                if self.cursor_window == Some(window_id) {
                    self.cursor_window = None;
                    self.state.clear_cursor();
                }
            },
            InputEvent::CursorMoved { window_id, x, y } => {
                self.cursor_entered(window_id);
                self.state.move_cursor(x, y);
            },
            InputEvent::KeyboardKeyPressed { window_id, key } if self.focused_window == Some(window_id) => self.state.press_key(key),
            InputEvent::KeyboardKeyReleased { window_id, key } if self.focused_window == Some(window_id) => self.state.release_key(key),
            InputEvent::MouseButtonPressed { window_id, button } if self.focused_window == Some(window_id) => self.state.press_mouse_button(button),
            InputEvent::MouseButtonReleased { window_id, button } if self.focused_window == Some(window_id) => self.state.release_mouse_button(button),
            InputEvent::MouseMotion { x, y } if self.focused_window.is_some() => self.state.add_mouse_motion(x, y),
            InputEvent::MouseWheelLines { x, y, .. } => self.state.add_scroll_lines(x, y),
            InputEvent::MouseWheelPixels { x, y, .. } => self.state.add_scroll_pixels(x, y),
            _ => {},
        }
    }

    pub(crate) const fn state(&self) -> &InputState {
        &self.state
    }

    pub(crate) fn focus(&mut self, window_id: WindowId, focused: bool) {
        if focused {
            if self.focused_window == Some(window_id) {
                return;
            }

            self.focused_window = Some(window_id);
        } else if self.focused_window == Some(window_id) {
            self.focused_window = None;
        } else {
            return;
        }

        self.state.clear_transient();
        self.state.release_all();
        self.state.clear_cursor();
        self.cursor_window = None;
    }

    pub(crate) fn reset(&mut self) {
        *self = Self::new();
    }

    pub(crate) fn clear_transient(&mut self) {
        self.state.clear_transient();
    }

    fn cursor_entered(&mut self, window_id: WindowId) {
        if self.cursor_window != Some(window_id) {
            self.cursor_window = Some(window_id);
            self.state.clear_cursor();
        }
    }
}
