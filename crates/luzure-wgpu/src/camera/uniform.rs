use bytemuck::{Pod, Zeroable};
use luzure_render::Camera;

#[repr(C, align(16))]
#[derive(Clone, Copy, Pod, Zeroable)]
pub(super) struct CameraUniform {
    view_projection: [[f32; 4]; 4],
}

impl CameraUniform {
    pub(super) fn new(camera: &Camera) -> Self {
        Self {
            view_projection: *camera.view_projection().columns(),
        }
    }
}
