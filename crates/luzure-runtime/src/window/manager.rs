use luzure_backend::{Window, backend::{BackendError, BackendHandle}, window::{WindowDescriptor, WindowEvent, WindowEventKind, WindowId}};
use luzure_ecs::{Entity, Registry};
use luzure_render::Renderer;

use std::collections::HashMap;

use crate::{runtime::RuntimeError, window::{WindowPlan, WindowState}};

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

    pub(crate) fn apply<R: Renderer<Surface = S>, H: BackendHandle>(&mut self, plan: &mut WindowPlan, registry: &mut Registry, renderer: &mut R, handle: &mut H)
        -> Result<(), RuntimeError>
    {
        for (window, descriptor) in plan.creates() {
            self.create(registry, renderer, handle, window, descriptor)?;
        }

        for window in plan.destroys() {
            self.destroy(registry, handle, window)?;
        }

        Ok(())
    }

    pub(crate) fn create<R: Renderer<Surface = S>, H: BackendHandle>(&mut self, registry: &mut Registry, renderer: &mut R, handle: &mut H, entity: Entity, descriptor: WindowDescriptor)
        -> Result<(), RuntimeError>
    {
        if !registry.contains(entity) {
            return Err(BackendError::InvalidWindow.into());
        }

        let window = handle.create_window(descriptor.clone())?;
        let window_id = window.id();
        let (width, height) = window.inner_size();
        let surface = renderer.create_surface(window.clone(), (width, height))?;

        let _ = registry.insert(entity, window)?;
        registry.insert(entity, WindowState::new(descriptor, width, height))?;
        self.add(window_id, entity, surface);

        Ok(())
    }

    pub(crate) fn destroy<H: BackendHandle>(&mut self, registry: &mut Registry, handle: &mut H, entity: Entity)
        -> Result<(), RuntimeError>
    {
        let window_id = self.window_id(entity)
            .ok_or(BackendError::InvalidWindow)?;

        self.surfaces.remove(&window_id);
        self.entities.remove(&window_id);
        self.window_ids.remove(&entity);
        registry.despawn(entity);
        handle.destroy_window(window_id)?;

        Ok(())
    }

    pub(crate) fn destroy_all<H: BackendHandle>(&mut self, registry: &mut Registry, handle: &mut H)
        -> Result<(), RuntimeError>
    {
        let entities: Vec<Entity> = self.window_ids.keys().copied().collect();

        for entity in entities {
            self.destroy(registry, handle, entity)?;
        }

        Ok(())
    }

    pub(crate) fn request_redraws(&self, registry: &Registry) {
        for entity in self.entities.values() {
            if let Some(window) = registry.get::<Window>(*entity) {
                window.request_redraw();
            }
        }
    }

    pub(crate) fn surface(&self, window_id: WindowId) -> Option<&S> {
        self.surfaces.get(&window_id)
    }

    pub(crate) fn surface_mut(&mut self, window_id: WindowId) -> Option<&mut S> {
        self.surfaces.get_mut(&window_id)
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
