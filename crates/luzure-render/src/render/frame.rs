use crate::{CameraMatrices, MeshBatch, MeshInstance};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderFrame<'a> {
    camera_matrices: &'a CameraMatrices,
    instances: &'a [MeshInstance],
    mesh_batches: &'a [MeshBatch],
}

impl<'a> RenderFrame<'a> {
    pub const fn new(camera_matrices: &'a CameraMatrices, instances: &'a [MeshInstance], mesh_batches: &'a [MeshBatch]) -> Self {
        Self {
            camera_matrices,
            instances,
            mesh_batches,
        }
    }

    pub const fn camera_matrices(&self) -> &CameraMatrices {
        self.camera_matrices
    }

    pub const fn instances(&self) -> &[MeshInstance] {
        self.instances
    }

    pub const fn mesh_batches(&self) -> &[MeshBatch] {
        self.mesh_batches
    }
}
