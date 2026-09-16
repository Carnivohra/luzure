use crate::{MeshBatch, MeshInstance, RenderView};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderFrame<'a> {
    instances: &'a [MeshInstance],
    mesh_batches: &'a [MeshBatch],
    views: &'a [RenderView],
}

impl<'a> RenderFrame<'a> {
    pub(crate) const fn new(views: &'a [RenderView], instances: &'a [MeshInstance], mesh_batches: &'a [MeshBatch]) -> Self {
        Self {
            instances,
            mesh_batches,
            views,
        }
    }

    pub const fn instances(&self) -> &[MeshInstance] {
        self.instances
    }

    pub const fn mesh_batches(&self) -> &[MeshBatch] {
        self.mesh_batches
    }

    pub const fn views(&self) -> &[RenderView] {
        self.views
    }
}
