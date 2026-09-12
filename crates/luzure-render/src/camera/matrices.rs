use luzure_math::Mat4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraMatrices {
    view: Mat4,
    projection: Mat4,
}

impl CameraMatrices {
    pub const IDENTITY: Self = Self::new(Mat4::IDENTITY, Mat4::IDENTITY);

    pub const fn new(view: Mat4, projection: Mat4) -> Self {
        Self {
            view,
            projection,
        }
    }

    pub const fn view(&self) -> &Mat4 {
        &self.view
    }

    pub const fn projection(&self) -> &Mat4 {
        &self.projection
    }

    #[inline]
    pub fn view_projection(&self) -> Mat4 {
        self.projection * self.view
    }
}
