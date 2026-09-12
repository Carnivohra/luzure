mod mesh;
mod system;

pub use system::RenderExtractSystem;

use mesh::extract_meshes;

use luzure_ecs::Registry;
use luzure_render::{MeshHandle, MeshInstance, RenderScene, render::RenderError};

pub struct RenderExtraction {
    mesh_instances: Vec<(MeshHandle, MeshInstance)>,
    systems: Vec<RenderExtractSystem>,
}

impl RenderExtraction {
    pub const fn new() -> Self {
        Self {
            mesh_instances: Vec::new(),
            systems: Vec::new(),
        }
    }

    pub fn add_system(&mut self, system: RenderExtractSystem) {
        self.systems.push(system);
    }

    pub fn run(&mut self, registry: &Registry, scene: &mut RenderScene)
        -> Result<(), RenderError>
    {
        extract_meshes(registry, scene, &mut self.mesh_instances)?;

        for system in &self.systems {
            system(registry, scene)?;
        }

        Ok(())
    }
}
