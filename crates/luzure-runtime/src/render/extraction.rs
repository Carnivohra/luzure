mod system;

pub use system::RenderExtractSystem;

use luzure_ecs::Registry;
use luzure_render::{RenderScene, render::RenderError};

pub struct RenderExtraction {
    systems: Vec<RenderExtractSystem>,
}

impl RenderExtraction {
    pub const fn new() -> Self {
        Self { systems: Vec::new() }
    }

    pub fn add_system(&mut self, system: RenderExtractSystem) {
        self.systems.push(system);
    }

    pub fn run(&self, registry: &Registry, scene: &mut RenderScene)
        -> Result<(), RenderError>
    {
        for system in &self.systems {
            system(registry, scene)?;
        }

        Ok(())
    }
}
