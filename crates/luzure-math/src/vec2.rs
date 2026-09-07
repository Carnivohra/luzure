use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub const fn splat(value: f32) -> Self {
        Self::new(value, value)
    }

    #[inline]
    pub const fn dot(self, right: Self) -> f32 {
        self.x * right.x + self.y * right.y
    }

    #[inline]
    pub const fn length_squared(self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        self * self.length().recip()
    }
}

impl Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, right: Self) -> Self::Output {
        Self::new(self.x + right.x, self.y + right.y)
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, right: Self) {
        self.x += right.x;
        self.y += right.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, right: Self) -> Self::Output {
        Self::new(self.x - right.x, self.y - right.y)
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, right: Self) {
        self.x -= right.x;
        self.y -= right.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, right: f32) -> Self::Output {
        Self::new(self.x * right, self.y * right)
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, right: f32) {
        self.x *= right;
        self.y *= right;
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn div(self, right: f32) -> Self::Output {
        Self::new(self.x / right, self.y / right)
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, right: f32) {
        self.x /= right;
        self.y /= right;
    }
}

impl Neg for Vec2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}
