use crate::Vec3;

use std::ops::{Mul, MulAssign};

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    columns: [[f32; 4]; 4],
}

impl Mat4 {
    pub const ZERO: Self = Self::new([[0.0; 4]; 4]);
    pub const IDENTITY: Self = Self::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    pub const fn new(columns: [[f32; 4]; 4]) -> Self {
        Self { columns }
    }

    pub const fn from_translation(translation: Vec3) -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [translation.x, translation.y, translation.z, 1.0],
        ])
    }

    pub const fn from_scale(scale: Vec3) -> Self {
        Self::new([
            [scale.x, 0.0, 0.0, 0.0],
            [0.0, scale.y, 0.0, 0.0],
            [0.0, 0.0, scale.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub const fn columns(&self) -> &[[f32; 4]; 4] {
        &self.columns
    }

    #[inline]
    fn multiply_column(&self, right: [f32; 4]) -> [f32; 4] {
        [
            self.columns[0][0] * right[0]
                + self.columns[1][0] * right[1]
                + self.columns[2][0] * right[2]
                + self.columns[3][0] * right[3],

            self.columns[0][1] * right[0]
                + self.columns[1][1] * right[1]
                + self.columns[2][1] * right[2]
                + self.columns[3][1] * right[3],

            self.columns[0][2] * right[0]
                + self.columns[1][2] * right[1]
                + self.columns[2][2] * right[2]
                + self.columns[3][2] * right[3],

            self.columns[0][3] * right[0]
                + self.columns[1][3] * right[1]
                + self.columns[2][3] * right[2]
                + self.columns[3][3] * right[3],
        ]
    }
}

impl Mul for Mat4 {
    type Output = Self;

    #[inline]
    fn mul(self, right: Self) -> Self::Output {
        Self::new([
            self.multiply_column(right.columns[0]),
            self.multiply_column(right.columns[1]),
            self.multiply_column(right.columns[2]),
            self.multiply_column(right.columns[3]),
        ])
    }
}

impl MulAssign for Mat4 {
    #[inline]
    fn mul_assign(&mut self, right: Self) {
        *self = *self * right;
    }
}
