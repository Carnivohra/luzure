use luzure_backend::input::InputEvent;
use luzure_input::input::InputState;

pub(crate) struct InputRuntime {
    state: InputState,
}

impl InputRuntime {
    pub(crate) const fn new() -> Self {
        Self { state: InputState::new() }
    }

    pub(crate) fn update(&mut self, event: InputEvent) {
        match event {
            InputEvent::CursorEntered { .. } => {}
            InputEvent::CursorLeft { .. } => self.state.clear_cursor(),
            InputEvent::CursorMoved { x, y, .. } => self.state.move_cursor(x, y),
            InputEvent::KeyboardKeyPressed { key, .. } => self.state.press_key(key),
            InputEvent::KeyboardKeyReleased { key, .. } => self.state.release_key(key),
            InputEvent::MouseButtonPressed { button, .. } => self.state.press_mouse_button(button),
            InputEvent::MouseButtonReleased { button, .. } => { self.state.release_mouse_button(button) }
            InputEvent::MouseMotion { x, y } => self.state.add_mouse_motion(x, y),
            InputEvent::MouseWheelLines { x, y, .. } => self.state.add_scroll_lines(x, y),
            InputEvent::MouseWheelPixels { x, y, .. } => self.state.add_scroll_pixels(x, y),
        }
    }

    pub(crate) const fn state(&self) -> &InputState {
        &self.state
    }

    pub(crate) fn release_all(&mut self) {
        self.state.release_all();
    }

    pub(crate) fn reset(&mut self) {
        self.state.reset();
    }

    pub(crate) fn clear_transient(&mut self) {
        self.state.clear_transient();
    }
}
