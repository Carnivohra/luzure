use crate::MeshHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeshBatch {
    mesh: MeshHandle,
    first_instance: u32,
    instance_count: u32,
}

impl MeshBatch {
    pub const fn new(mesh: MeshHandle, first_instance: u32, instance_count: u32) -> Self {
        Self {
            mesh,
            first_instance,
            instance_count,
        }
    }

    pub const fn mesh(self) -> MeshHandle {
        self.mesh
    }

    pub const fn first_instance(self) -> u32 {
        self.first_instance
    }

    pub const fn instance_count(self) -> u32 {
        self.instance_count
    }
}
