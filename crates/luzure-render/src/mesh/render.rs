use crate::MeshHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeshRender {
    mesh: MeshHandle,
}

impl MeshRender {
    pub const fn new(mesh: MeshHandle) -> Self {
        Self { mesh }
    }

    pub const fn mesh(self) -> MeshHandle {
        self.mesh
    }
}
