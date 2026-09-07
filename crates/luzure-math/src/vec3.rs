use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub const fn splat(value: f32) -> Self {
        Self::new(value, value, value)
    }

    #[inline]
    pub const fn dot(self, right: Self) -> f32 {
        self.x * right.x + self.y * right.y + self.z * right.z
    }

    #[inline]
    pub const fn cross(self, right: Self) -> Self {
        Self::new(
            self.y * right.z - self.z * right.y,
            self.z * right.x - self.x * right.z,
            self.x * right.y - self.y * right.x,
        )
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

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, right: Self) -> Self::Output {
        Self::new(self.x + right.x, self.y + right.y, self.z + right.z)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, right: Self) {
        self.x += right.x;
        self.y += right.y;
        self.z += right.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, right: Self) -> Self::Output {
        Self::new(self.x - right.x, self.y - right.y, self.z - right.z)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, right: Self) {
        self.x -= right.x;
        self.y -= right.y;
        self.z -= right.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, right: f32) -> Self::Output {
        Self::new(self.x * right, self.y * right, self.z * right)
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, right: f32) {
        self.x *= right;
        self.y *= right;
        self.z *= right;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, right: f32) -> Self::Output {
        Self::new(self.x / right, self.y / right, self.z / right)
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, right: f32) {
        self.x /= right;
        self.y /= right;
        self.z /= right;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}
