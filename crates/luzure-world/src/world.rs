use luzure_ecs::{Entity, Registry};

use crate::Camera;

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

    pub fn create_camera(&mut self, camera: Camera) -> Entity {
        self.registry.spawn(camera)
    }

    pub fn camera(&self, entity: Entity) -> Option<&Camera> {
        self.registry.get(entity)
    }

    pub fn camera_mut(&mut self, entity: Entity) -> Option<&mut Camera> {
        self.registry.get_mut(entity)
    }
}
