use crate::Camera;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderFrame<'a> {
    camera: &'a Camera,
}

impl<'a> RenderFrame<'a> {
    pub const fn new(camera: &'a Camera) -> Self {
        Self {
            camera,
        }
    }

    pub const fn camera(&self) -> &Camera {
        self.camera
    }
}
