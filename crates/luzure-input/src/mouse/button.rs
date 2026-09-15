#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

impl MouseButton {
    pub(crate) const fn mask(self) -> u8 {
        1 << self as u8
    }
}

const _: () = assert!((MouseButton::Forward as u8) < u8::BITS as u8);
