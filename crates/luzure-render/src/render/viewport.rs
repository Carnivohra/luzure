#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Viewport {
    pub const FULL: Self = Self::normalized(0.0, 0.0, 1.0, 1.0);

    pub const fn normalized(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub const fn x(&self) -> f32 {
        self.x
    }

    pub const fn y(&self) -> f32 {
        self.y
    }

    pub const fn width(&self) -> f32 {
        self.width
    }

    pub const fn height(&self) -> f32 {
        self.height
    }
}
