use luzure_ecs::{Bundle, Entity, Registry};

use super::{MainSchedule, MainSystem};

pub struct MainContext<'a> {
    registry: &'a mut Registry,
    schedule: &'a mut MainSchedule,
}

impl<'a> MainContext<'a> {
    pub(crate) const fn new(registry: &'a mut Registry, schedule: &'a mut MainSchedule) -> Self {
        Self { registry, schedule }
    }

    pub fn insert_resource<T: Send + Sync + 'static>(&mut self, resource: T) -> Option<T> {
        self.registry.insert_resource(resource)
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> Entity {
        self.registry.spawn_bundle(bundle)
    }

    pub fn spawn_component<T: Send + Sync + 'static>(&mut self, component: T) -> Entity {
        self.registry.spawn(component)
    }

    pub fn add_system(&mut self, system: MainSystem) {
        self.schedule.add_system(system);
    }
}
