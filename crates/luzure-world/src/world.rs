use luzure_ecs::Registry;

pub struct World {
    registry: Registry,
}

impl World {
    pub fn new() -> Self {
        Self {
            registry: Registry::new(),
        }
    }

    pub const fn registry(&self) -> &Registry {
        &self.registry
    }

    pub const fn registry_mut(&mut self) -> &mut Registry {
        &mut self.registry
    }
}
