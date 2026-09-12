use bytemuck::{Pod, Zeroable};
use luzure_render::CameraMatrices;

#[repr(C, align(16))]
#[derive(Clone, Copy, Pod, Zeroable)]
pub(super) struct CameraUniform {
    view_projection: [[f32; 4]; 4],
}

impl CameraUniform {
    pub(super) fn new(camera_matrices: &CameraMatrices) -> Self {
        Self {
            view_projection: *camera_matrices.view_projection().columns(),
        }
    }
}
