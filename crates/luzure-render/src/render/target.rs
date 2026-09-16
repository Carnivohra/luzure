#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderTarget(u64);

impl RenderTarget {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}
