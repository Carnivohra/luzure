use crate::{Camera, MeshBatch, MeshInstance};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderFrame<'a> {
    camera: &'a Camera,
    instances: &'a [MeshInstance],
    mesh_batches: &'a [MeshBatch],
}

impl<'a> RenderFrame<'a> {
    pub const fn new(camera: &'a Camera, instances: &'a [MeshInstance], mesh_batches: &'a [MeshBatch]) -> Self {
        Self {
            camera,
            instances,
            mesh_batches,
        }
    }

    pub const fn camera(&self) -> &Camera {
        self.camera
    }

    pub const fn instances(&self) -> &[MeshInstance] {
        self.instances
    }

    pub const fn mesh_batches(&self) -> &[MeshBatch] {
        self.mesh_batches
    }
}
