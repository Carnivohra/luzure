use luzure_render::{MeshDescriptor, MeshHandle, Renderer, render::RenderError};

pub(crate) struct RenderPlan {
    next_mesh: u64,
    creates: Vec<(MeshHandle, MeshDescriptor)>,
    destroys: Vec<MeshHandle>,
}

impl RenderPlan {
    pub(crate) const fn new() -> Self {
        Self {
            next_mesh: 0,
            creates: Vec::new(),
            destroys: Vec::new(),
        }
    }

    pub(crate) fn create(&mut self, descriptor: MeshDescriptor) -> Result<MeshHandle, RenderError> {
        let next_mesh = self.next_mesh.checked_add(1)
            .ok_or(RenderError::MeshCapacityExceeded)?;
        let mesh = MeshHandle::new(self.next_mesh);

        self.next_mesh = next_mesh;
        self.creates.push((mesh, descriptor));

        Ok(mesh)
    }

    pub(crate) fn destroy(&mut self, mesh: MeshHandle) {
        self.destroys.push(mesh);
    }

    pub(crate) fn apply<R: Renderer>(&mut self, renderer: &mut R) -> Result<(), RenderError> {
        for (mesh, descriptor) in self.creates.drain(..) {
            renderer.create_mesh(mesh, descriptor)?;
        }

        for mesh in self.destroys.drain(..) {
            renderer.destroy_mesh(mesh)?;
        }

        Ok(())
    }
}
