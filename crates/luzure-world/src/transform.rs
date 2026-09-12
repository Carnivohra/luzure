use luzure_math::Mat4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    matrix: Mat4,
}

impl Transform {
    pub const IDENTITY: Self = Self::new(Mat4::IDENTITY);

    pub const fn new(matrix: Mat4) -> Self {
        Self { matrix }
    }

    pub const fn matrix(&self) -> &Mat4 {
        &self.matrix
    }

    pub fn set_matrix(&mut self, matrix: Mat4) {
        self.matrix = matrix;
    }
}
