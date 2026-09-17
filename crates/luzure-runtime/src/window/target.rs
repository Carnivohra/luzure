use luzure_backend::Window;
use luzure_ecs::Entity;

pub(crate) struct WindowTarget<S> {
    entity: Entity,
    has_size: bool,
    occluded: bool,
    surface: Option<S>,
    visible: bool,
    window: Window,
}

impl<S> WindowTarget<S> {
    pub(crate) const fn new(entity: Entity, window: Window, surface: Option<S>, visible: bool, size: (u32, u32)) -> Self {
        Self {
            entity,
            has_size: size.0 > 0 && size.1 > 0,
            occluded: false,
            surface,
            visible,
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

    pub(crate) const fn should_redraw(&self) -> bool {
        self.visible && self.has_size && !self.occluded && self.surface.is_some()
    }

    pub(crate) const fn resize(&mut self, width: u32, height: u32) {
        self.has_size = width > 0 && height > 0;
    }

    pub(crate) const fn set_occluded(&mut self, occluded: bool) {
        self.occluded = occluded;
    }

    pub(crate) fn set_surface(&mut self, surface: S) {
        self.surface = Some(surface);
    }

    pub(crate) fn suspend(&mut self) {
        self.surface = None;
    }
}
