use crate::{Camera, MeshBatch, MeshHandle, MeshInstance};

use super::{RenderError, RenderFrame};

pub struct RenderScene {
    instances: Vec<MeshInstance>,
    mesh_batches: Vec<MeshBatch>,
}

impl RenderScene {
    pub const fn new() -> Self {
        Self {
            instances: Vec::new(),
            mesh_batches: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.instances.clear();
        self.mesh_batches.clear();
    }

    pub fn push_batch(&mut self, mesh: MeshHandle, instances: &[MeshInstance])
        -> Result<(), RenderError>
    {
        if instances.is_empty() {
            return Ok(());
        }

        let first_instance = u32::try_from(self.instances.len())
            .map_err(|_| RenderError::InstanceCapacityExceeded)?;
        let instance_count = u32::try_from(instances.len())
            .map_err(|_| RenderError::InstanceCapacityExceeded)?;

        first_instance.checked_add(instance_count)
            .ok_or(RenderError::InstanceCapacityExceeded)?;

        self.instances.extend_from_slice(instances);
        self.mesh_batches.push(MeshBatch::new(
            mesh,
            first_instance,
            instance_count,
        ));

        Ok(())
    }

    pub fn instances(&self) -> &[MeshInstance] {
        &self.instances
    }

    pub fn mesh_batches(&self) -> &[MeshBatch] {
        &self.mesh_batches
    }

    pub fn frame<'a>(&'a self, camera: &'a Camera) -> RenderFrame<'a> {
        RenderFrame::new(
            camera,
            &self.instances,
            &self.mesh_batches,
        )
    }
}
