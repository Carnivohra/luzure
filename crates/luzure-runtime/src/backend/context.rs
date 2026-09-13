use luzure_backend::{backend::BackendError, window::WindowDescriptor};
use luzure_ecs::{Entity, Registry};

use crate::{runtime::RuntimeError, window::{PrimaryWindow, WindowPlan}};

pub struct BackendContext<'a> {
    runtime_registry: &'a mut Registry,
    window_plan: &'a mut WindowPlan,
}

impl<'a> BackendContext<'a> {
    pub(crate) const fn new(runtime_registry: &'a mut Registry, window_plan: &'a mut WindowPlan) -> Self {
        Self {
            runtime_registry,
            window_plan,
        }
    }

    pub fn create_window(&mut self, descriptor: WindowDescriptor) -> Result<Entity, RuntimeError> {
        let window = self.runtime_registry.spawn_empty();

        self.window_plan.create(window, descriptor);

        Ok(window)
    }

    pub fn destroy_window(&mut self, window: Entity) -> Result<(), RuntimeError> {
        if !self.runtime_registry.contains(window) {
            return Err(BackendError::InvalidWindow.into());
        }

        self.window_plan.destroy(window);

        Ok(())
    }

    pub fn set_primary_window(&mut self, window: Entity) -> Result<(), RuntimeError> {
        if !self.runtime_registry.contains(window) {
            return Err(BackendError::InvalidWindow.into());
        }

        if self.runtime_registry.contains_component::<PrimaryWindow>(window) {
            return Ok(());
        }

        let primary = self.runtime_registry.query::<PrimaryWindow>()
            .next()
            .map(|(entity, _)| entity);

        if let Some(primary) = primary {
            let _ = self.runtime_registry.remove::<PrimaryWindow>(primary);
        }

        let _ = self.runtime_registry.insert(window, PrimaryWindow)?;

        Ok(())
    }
}
