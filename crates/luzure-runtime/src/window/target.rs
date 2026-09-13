use luzure_backend::Window;
use luzure_ecs::Entity;

pub(crate) struct WindowTarget<S> {
    entity: Entity,
    surface: Option<S>,
    window: Window,
}

impl<S> WindowTarget<S> {
    pub(crate) const fn new(entity: Entity, window: Window, surface: S) -> Self {
        Self {
            entity,
            surface: Some(surface),
            window,
        }
    }

    pub(crate) const fn entity(&self) -> Entity {
        self.entity
    }

    pub(crate) const fn window(&self) -> &Window {
        &self.window
    }

    pub(crate) const fn surface_mut(&mut self) -> Option<&mut S> {
        self.surface.as_mut()
    }

    pub(crate) fn set_surface(&mut self, surface: S) {
        self.surface = Some(surface);
    }

    pub(crate) fn suspend(&mut self) {
        self.surface = None;
    }
}
