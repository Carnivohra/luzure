use crate::{keyboard::KeyboardKey, mouse::MouseButton};

#[derive(Debug, Clone, Copy, Default)]
pub struct InputState {
    keys_down: u128,
    keys_pressed: u128,
    keys_released: u128,

    mouse_buttons_down: u8,
    mouse_buttons_pressed: u8,
    mouse_buttons_released: u8,

    cursor_position: Option<(f64, f64)>,
    cursor_delta: (f64, f64),
    mouse_motion: (f64, f64),
    scroll_lines: (f32, f32),
    scroll_pixels: (f64, f64),
}

impl InputState {
    pub const fn new() -> Self {
        Self {
            keys_down: 0,
            keys_pressed: 0,
            keys_released: 0,
            mouse_buttons_down: 0,
            mouse_buttons_pressed: 0,
            mouse_buttons_released: 0,
            cursor_position: None,
            cursor_delta: (0.0, 0.0),
            mouse_motion: (0.0, 0.0),
            scroll_lines: (0.0, 0.0),
            scroll_pixels: (0.0, 0.0),
        }
    }

    pub const fn key_down(&self, key: KeyboardKey) -> bool {
        self.keys_down & key.mask() != 0
    }

    pub const fn key_pressed(&self, key: KeyboardKey) -> bool {
        self.keys_pressed & key.mask() != 0
    }

    pub const fn key_released(&self, key: KeyboardKey) -> bool {
        self.keys_released & key.mask() != 0
    }

    pub const fn mouse_button_down(&self, button: MouseButton) -> bool {
        self.mouse_buttons_down & button.mask() != 0
    }

    pub const fn mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_pressed & button.mask() != 0
    }

    pub const fn mouse_button_released(&self, button: MouseButton) -> bool {
        self.mouse_buttons_released & button.mask() != 0
    }

    pub const fn cursor_position(&self) -> Option<(f64, f64)> {
        self.cursor_position
    }

    pub const fn cursor_delta(&self) -> (f64, f64) {
        self.cursor_delta
    }

    pub const fn mouse_motion(&self) -> (f64, f64) {
        self.mouse_motion
    }

    pub const fn scroll_lines(&self) -> (f32, f32) {
        self.scroll_lines
    }

    pub const fn scroll_pixels(&self) -> (f64, f64) {
        self.scroll_pixels
    }

    pub fn press_key(&mut self, key: KeyboardKey) {
        let mask = key.mask();

        if self.keys_down & mask == 0 {
            self.keys_pressed |= mask;
            self.keys_down |= mask;
        }
    }

    pub fn release_key(&mut self, key: KeyboardKey) {
        let mask = key.mask();

        if self.keys_down & mask != 0 {
            self.keys_released |= mask;
            self.keys_down &= !mask;
        }
    }

    pub fn press_mouse_button(&mut self, button: MouseButton) {
        let mask = button.mask();

        if self.mouse_buttons_down & mask == 0 {
            self.mouse_buttons_pressed |= mask;
            self.mouse_buttons_down |= mask;
        }
    }

    pub fn release_mouse_button(&mut self, button: MouseButton) {
        let mask = button.mask();

        if self.mouse_buttons_down & mask != 0 {
            self.mouse_buttons_released |= mask;
            self.mouse_buttons_down &= !mask;
        }
    }

    pub fn move_cursor(&mut self, x: f64, y: f64) {
        if let Some((previous_x, previous_y)) = self.cursor_position {
            self.cursor_delta.0 += x - previous_x;
            self.cursor_delta.1 += y - previous_y;
        }

        self.cursor_position = Some((x, y));
    }

    pub fn clear_cursor(&mut self) {
        self.cursor_position = None;
    }

    pub fn add_mouse_motion(&mut self, x: f64, y: f64) {
        self.mouse_motion.0 += x;
        self.mouse_motion.1 += y;
    }

    pub fn add_scroll_lines(&mut self, x: f32, y: f32) {
        self.scroll_lines.0 += x;
        self.scroll_lines.1 += y;
    }

    pub fn add_scroll_pixels(&mut self, x: f64, y: f64) {
        self.scroll_pixels.0 += x;
        self.scroll_pixels.1 += y;
    }

    pub fn release_all(&mut self) {
        self.keys_released |= self.keys_down;
        self.keys_down = 0;
        self.mouse_buttons_released |= self.mouse_buttons_down;
        self.mouse_buttons_down = 0;
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn clear_transient(&mut self) {
        self.keys_pressed = 0;
        self.keys_released = 0;
        self.mouse_buttons_pressed = 0;
        self.mouse_buttons_released = 0;
        self.cursor_delta = (0.0, 0.0);
        self.mouse_motion = (0.0, 0.0);
        self.scroll_lines = (0.0, 0.0);
        self.scroll_pixels = (0.0, 0.0);
    }
}
