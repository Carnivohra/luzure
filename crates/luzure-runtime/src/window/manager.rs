use luzure_backend::{Window, window::{WindowEvent, WindowEventKind, WindowId}};
use luzure_ecs::{Entity, Registry};

use std::collections::HashMap;

use crate::window::WindowState;

pub struct WindowManager<S> {
    entities: HashMap<WindowId, Entity>,
    surfaces: HashMap<WindowId, S>,
    window_ids: HashMap<Entity, WindowId>,
}

impl<S> WindowManager<S> {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            surfaces: HashMap::new(),
            window_ids: HashMap::new(),
        }
    }

    pub fn entity(&self, window_id: WindowId) -> Option<Entity> {
        self.entities.get(&window_id).copied()
    }

    pub fn window_id(&self, entity: Entity) -> Option<WindowId> {
        self.window_ids.get(&entity).copied()
    }

    pub(crate) fn surface(&self, window_id: WindowId) -> Option<&S> {
        self.surfaces.get(&window_id)
    }

    pub fn set_title(&self, registry: &mut Registry, entity: Entity, title: &str) -> bool {
        let Some(window) = registry.get::<Window>(entity).cloned() else {
            return false;
        };

        let Some(state) = registry.get_mut::<WindowState>(entity) else {
            return false;
        };

        window.set_title(title);
        state.set_title(title);

        true
    }

    pub(crate) fn add(&mut self, window_id: WindowId, entity: Entity, surface: S) {
        debug_assert!(!self.entities.contains_key(&window_id));
        debug_assert!(!self.window_ids.contains_key(&entity));

        self.entities.insert(window_id, entity);
        self.surfaces.insert(window_id, surface);
        self.window_ids.insert(entity, window_id);
    }

    pub(crate) fn synchronize(&self, registry: &mut Registry, event: WindowEvent) {
        match event.kind {
            WindowEventKind::CloseRequested => return,
            WindowEventKind::RedrawRequested => return,
            _ => {},
        }

        let Some(entity) = self.entity(event.window_id) else {
            return;
        };

        let Some(state) = registry.get_mut::<WindowState>(entity) else {
            return;
        };

        match event.kind {
            WindowEventKind::Resized { width, height } => state.resize(width, height),
            WindowEventKind::Focused { focused } => state.set_focused(focused),
            _ => {},
        }
    }
}
