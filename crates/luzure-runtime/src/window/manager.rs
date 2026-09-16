use luzure_backend::{backend::{BackendError, BackendHandle}, window::{WindowDescriptor, WindowEvent, WindowEventKind, WindowId}};
use luzure_ecs::{Entity, Registry};
use luzure_render::{RenderTarget, Renderer};

use std::collections::HashMap;

use crate::{render::render_target, runtime::RuntimeError, window::{PrimaryWindow, WindowPlan, WindowState, WindowTarget}};

pub(crate) struct WindowManager<S> {
    plan: WindowPlan,
    targets: HashMap<WindowId, WindowTarget<S>>,
    window_ids: HashMap<Entity, WindowId>,
}

impl<S> WindowManager<S> {
    pub(crate) fn new() -> Self {
        Self {
            plan: WindowPlan::new(),
            targets: HashMap::new(),
            window_ids: HashMap::new(),
        }
    }

    pub(crate) fn window_id(&self, entity: Entity) -> Option<WindowId> {
        self.window_ids.get(&entity).copied()
    }

    pub(crate) const fn plan_mut(&mut self) -> &mut WindowPlan {
        &mut self.plan
    }

    pub(crate) fn apply<R: Renderer<Surface = S>, H: BackendHandle>(&mut self, registry: &mut Registry, renderer: &mut R, handle: &mut H)
        -> Result<(), RuntimeError>
    {
        let mut plan = std::mem::replace(&mut self.plan, WindowPlan::new());

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
        let visible = descriptor.visible;

        registry.insert(entity, WindowState::new(descriptor, width, height))?;
        self.add(window_id, WindowTarget::new(entity, window, surface, visible, (width, height)));

        Ok(())
    }

    pub(crate) fn destroy<H: BackendHandle>(&mut self, registry: &mut Registry, handle: &mut H, entity: Entity)
        -> Result<(), RuntimeError>
    {
        let window_id = self.window_id(entity)
            .ok_or(BackendError::InvalidWindow)?;

        self.targets.remove(&window_id);
        self.window_ids.remove(&entity);
        registry.despawn(entity);
        handle.destroy_window(window_id)?;

        Ok(())
    }

    pub(crate) fn destroy_all<H: BackendHandle>(&mut self, registry: &mut Registry, handle: &mut H)
        -> Result<(), RuntimeError>
    {
        for (entity, window_id) in self.window_ids.drain() {
            self.targets.remove(&window_id);
            registry.despawn(entity);
            handle.destroy_window(window_id)?;
        }

        Ok(())
    }

    pub(crate) fn close_requested<H: BackendHandle>(&mut self, registry: &mut Registry, handle: &mut H, window_id: WindowId)
        -> Result<(), RuntimeError>
    {
        let target = self.targets.get(&window_id)
            .ok_or(BackendError::InvalidWindow)?;
        let window = target.entity();

        if self.targets.len() == 1 || registry.contains_component::<PrimaryWindow>(window) {
            handle.exit()?;
            return Ok(());
        }

        self.destroy(registry, handle, window)
    }

    pub(crate) fn request_redraws(&self) {
        for target in self.targets.values() {
            if target.should_redraw() {
                target.window().request_redraw();
            }
        }
    }

    pub(crate) fn resume_surfaces<R: Renderer<Surface = S>>(&mut self, renderer: &mut R)
        -> Result<(), RuntimeError>
    {
        for target in self.targets.values_mut() {
            if target.surface_mut().is_some() {
                continue;
            }

            let window = target.window();
            let surface = renderer.create_surface(window.clone(), window.inner_size())?;

            target.set_surface(surface);
        }

        Ok(())
    }

    pub(crate) fn suspend_surfaces(&mut self) {
        for target in self.targets.values_mut() {
            target.suspend();
        }
    }

    pub(crate) fn surface_mut(&mut self, window_id: WindowId) -> Option<&mut S> {
        self.targets.get_mut(&window_id)?.surface_mut()
    }

    pub(crate) fn render_target_mut(&mut self, window_id: WindowId) -> Option<(RenderTarget, &mut S)> {
        let target = self.targets.get_mut(&window_id)?;
        let render_target = render_target(target.entity());
        let surface = target.surface_mut()?;

        Some((render_target, surface))
    }

    pub(crate) fn add(&mut self, window_id: WindowId, target: WindowTarget<S>) {
        let entity = target.entity();

        debug_assert!(!self.targets.contains_key(&window_id));
        debug_assert!(!self.window_ids.contains_key(&entity));

        self.targets.insert(window_id, target);
        self.window_ids.insert(entity, window_id);
    }

    pub(crate) fn synchronize(&mut self, registry: &mut Registry, event: WindowEvent) {
        match event.kind {
            WindowEventKind::CloseRequested => return,
            WindowEventKind::RedrawRequested => return,
            _ => {},
        }

        let Some(target) = self.targets.get_mut(&event.window_id) else {
            return;
        };
        let entity = target.entity();

        match event.kind {
            WindowEventKind::Resized { width, height } => target.resize(width, height),
            WindowEventKind::Occluded { occluded } => target.set_occluded(occluded),
            _ => {},
        }

        let Some(state) = registry.get_mut::<WindowState>(entity) else {
            return;
        };

        match event.kind {
            WindowEventKind::Resized { width, height } => state.resize(width, height),
            WindowEventKind::Focused { focused } => state.set_focused(focused),
            WindowEventKind::Occluded { occluded } => state.set_occluded(occluded),
            _ => {},
        }
    }
}
