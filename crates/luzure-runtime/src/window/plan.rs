use luzure_backend::window::WindowDescriptor;
use luzure_ecs::Entity;

pub(crate) struct WindowPlan {
    creates: Vec<(Entity, WindowDescriptor)>,
    destroys: Vec<Entity>,
}

impl WindowPlan {
    pub(crate) const fn new() -> Self {
        Self {
            creates: Vec::new(),
            destroys: Vec::new(),
        }
    }

    pub(crate) fn create(&mut self, window: Entity, descriptor: WindowDescriptor) {
        self.creates.push((window, descriptor));
    }

    pub(crate) fn destroy(&mut self, window: Entity) {
        self.destroys.push(window);
    }

    pub(crate) fn creates(&mut self) -> impl Iterator<Item = (Entity, WindowDescriptor)> + '_ {
        self.creates.drain(..)
    }

    pub(crate) fn destroys(&mut self) -> impl Iterator<Item = Entity> + '_ {
        self.destroys.drain(..)
    }
}
