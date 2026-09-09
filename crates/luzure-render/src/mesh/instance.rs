use luzure_math::Mat4;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeshInstance {
    transform: Mat4,
}

impl MeshInstance {
    pub const fn new(transform: Mat4) -> Self {
        Self {
            transform,
        }
    }

    pub const fn transform(&self) -> &Mat4 {
        &self.transform
    }
}
