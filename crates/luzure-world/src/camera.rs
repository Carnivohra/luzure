use luzure_math::Mat4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    view: Mat4,
    projection: Mat4,
}

impl Camera {
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

    pub fn set_view(&mut self, view: Mat4) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Mat4) {
        self.projection = projection;
    }
}
