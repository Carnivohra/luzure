use luzure_backend::{backend::BackendError, window::WindowDescriptor};
use luzure_ecs::{Entity, Registry};

use crate::{runtime::RuntimeError, window::{PrimaryWindow, WindowPlan}};

pub struct BackendContext<'a> {
    registry: &'a mut Registry,
    windows: &'a mut WindowPlan,
}

impl<'a> BackendContext<'a> {
    pub(crate) const fn new(registry: &'a mut Registry, windows: &'a mut WindowPlan) -> Self {
        Self {
            registry,
            windows,
        }
    }

    pub fn create_window(&mut self, descriptor: WindowDescriptor) -> Result<Entity, RuntimeError> {
        let window = self.registry.spawn_empty();

        self.windows.create(window, descriptor);

        Ok(window)
    }

    pub fn destroy_window(&mut self, window: Entity) -> Result<(), RuntimeError> {
        if !self.registry.contains(window) {
            return Err(BackendError::InvalidWindow.into());
        }

        self.windows.destroy(window);

        Ok(())
    }

    pub fn set_primary_window(&mut self, window: Entity) -> Result<(), RuntimeError> {
        if !self.registry.contains(window) {
            return Err(BackendError::InvalidWindow.into());
        }

        if self.registry.contains_component::<PrimaryWindow>(window) {
            return Ok(());
        }

        let primary = self.registry.query::<PrimaryWindow>()
            .next()
            .map(|(entity, _)| entity);

        if let Some(primary) = primary {
            let _ = self.registry.remove::<PrimaryWindow>(primary);
        }

        let _ = self.registry.insert(window, PrimaryWindow)?;

        Ok(())
    }
}
