#[derive(Default)]
pub struct InputState {
    _keys_down: u128,
    _keys_pressed: u128,
    _keys_released: u128,

    _mouse_buttons_down: u8,
    _mouse_buttons_pressed: u8,
    _mouse_buttons_released: u8,

    _cursor_position: Option<(f64, f64)>,
    _cursor_delta: (f64, f64),
    _mouse_motion: (f64, f64),
}
