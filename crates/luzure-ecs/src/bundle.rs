mod sealed {
    pub trait Sealed {}
}

use crate::Registry;

pub trait Bundle: sealed::Sealed {
    fn spawn(self, registry: &mut Registry) -> crate::Entity;
}

impl<A: Send + Sync + 'static, B: Send + Sync + 'static> sealed::Sealed for (A, B) {}

impl<A: Send + Sync + 'static, B: Send + Sync + 'static> Bundle for (A, B) {
    fn spawn(self, registry: &mut Registry) -> crate::Entity {
        registry.spawn_two(self.0, self.1)
    }
}

impl<A: Send + Sync + 'static, B: Send + Sync + 'static, C: Send + Sync + 'static> sealed::Sealed for (A, B, C) {}

impl<A: Send + Sync + 'static, B: Send + Sync + 'static, C: Send + Sync + 'static> Bundle for (A, B, C) {
    fn spawn(self, registry: &mut Registry) -> crate::Entity {
        registry.spawn_three(self.0, self.1, self.2)
    }
}
