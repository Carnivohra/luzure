use crate::{CameraMatrices, MeshBatch, MeshHandle, MeshInstance};

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

    pub fn push_instance(&mut self, mesh: MeshHandle, instance: MeshInstance)
        -> Result<(), RenderError>
    {
        let first_instance = u32::try_from(self.instances.len())
            .map_err(|_| RenderError::InstanceCapacityExceeded)?;

        first_instance.checked_add(1)
            .ok_or(RenderError::InstanceCapacityExceeded)?;

        if let Some(batch) = self.mesh_batches.last_mut().filter(|batch| batch.mesh() == mesh) {
            let instance_count = batch.instance_count().checked_add(1)
                .ok_or(RenderError::InstanceCapacityExceeded)?;

            batch.first_instance().checked_add(instance_count)
                .ok_or(RenderError::InstanceCapacityExceeded)?;

            *batch = MeshBatch::new(mesh, batch.first_instance(), instance_count);
        } else {
            self.mesh_batches.push(MeshBatch::new(mesh, first_instance, 1));
        }

        self.instances.push(instance);

        Ok(())
    }

    pub fn instances(&self) -> &[MeshInstance] {
        &self.instances
    }

    pub fn mesh_batches(&self) -> &[MeshBatch] {
        &self.mesh_batches
    }

    pub fn frame<'a>(&'a self, camera_matrices: &'a CameraMatrices) -> RenderFrame<'a> {
        RenderFrame::new(
            camera_matrices,
            &self.instances,
            &self.mesh_batches,
        )
    }
}
