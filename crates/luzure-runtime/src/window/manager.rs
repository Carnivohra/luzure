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

    pub(crate) fn focused_window(&self, registry: &Registry) -> Option<WindowId> {
        registry.query::<WindowState>()
            .find(|(_, state)| state.focused())
            .and_then(|(entity, _)| self.window_id(entity))
    }

    pub(crate) const fn plan_mut(&mut self) -> &mut WindowPlan {
        &mut self.plan
    }

    pub(crate) fn apply<R: Renderer<Surface = S>, H: BackendHandle>(&mut self, registry: &mut Registry, renderer: &mut R, handle: &mut H)
        -> Result<(), RuntimeError>
    {
        let mut plan = std::mem::replace(&mut self.plan, WindowPlan::new());

        let mut creates = plan.creates();

        while let Some((window, descriptor)) = creates.next() {
            if let Err(error) = self.create(registry, renderer, handle, window, descriptor) {
                for (window, _) in creates {
                    registry.despawn(window);
                }

                return Err(error);
            }
        }

        drop(creates);

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

        let window = match handle.create_window(descriptor.clone()) {
            Ok(window) => window,
            Err(error) => {
                registry.despawn(entity);
                return Err(error.into());
            },
        };
        let window_id = window.id();
        let (width, height) = window.inner_size();
        let surface = if width > 0 && height > 0 {
            match renderer.create_surface(window.clone(), (width, height)) {
                Ok(surface) => Some(surface),
                Err(error) => {
                    let _ = handle.destroy_window(window_id);
                    registry.despawn(entity);
                    return Err(error.into());
                },
            }
        } else {
            None
        };
        let visible = descriptor.visible;

        if let Err(error) = registry.insert(entity, WindowState::new(descriptor, width, height)) {
            drop(surface);
            let _ = handle.destroy_window(window_id);
            return Err(error.into());
        }
        self.add(window_id, WindowTarget::new(entity, window, surface, visible, (width, height)));

        Ok(())
    }

    pub(crate) fn destroy<H: BackendHandle>(&mut self, registry: &mut Registry, handle: &mut H, entity: Entity)
        -> Result<(), RuntimeError>
    {
        let window_id = self.window_id(entity)
            .ok_or(BackendError::InvalidWindow)?;

        if let Some(target) = self.targets.get_mut(&window_id) {
            target.suspend();
        }

        handle.destroy_window(window_id)?;
        self.targets.remove(&window_id);
        self.window_ids.remove(&entity);
        registry.despawn(entity);

        Ok(())
    }

    pub(crate) fn destroy_all<H: BackendHandle>(&mut self, registry: &mut Registry, handle: &mut H)
        -> Result<(), RuntimeError>
    {
        let mut first_error = None;

        for (entity, window_id) in self.window_ids.drain() {
            self.targets.remove(&window_id);
            registry.despawn(entity);
            if let Err(error) = handle.destroy_window(window_id) {
                first_error.get_or_insert(error);
            }
        }

        match first_error {
            Some(error) => Err(error.into()),
            None => Ok(()),
        }
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

            let (width, height) = target.window().inner_size();
            target.resize(width, height);

            if width == 0 || height == 0 {
                continue;
            }

            let surface = renderer.create_surface(target.window().clone(), (width, height))?;

            target.set_surface(surface);
        }

        Ok(())
    }

    pub(crate) fn suspend_surfaces(&mut self) {
        for target in self.targets.values_mut() {
            target.suspend();
        }
    }

    pub(crate) fn resize_surface<R: Renderer<Surface = S>>(&mut self, renderer: &mut R, window_id: WindowId, size: (u32, u32))
        -> Result<(), RuntimeError>
    {
        let Some(target) = self.targets.get_mut(&window_id) else {
            return Ok(());
        };

        target.resize(size.0, size.1);

        if size.0 == 0 || size.1 == 0 {
            return Ok(());
        }

        if let Some(surface) = target.surface_mut() {
            renderer.resize_surface(surface, size)?;
        } else {
            let surface = renderer.create_surface(target.window().clone(), size)?;
            target.set_surface(surface);
        }

        Ok(())
    }

    pub(crate) fn render_target_mut(&mut self, window_id: WindowId) -> Option<(RenderTarget, &mut S)> {
        let target = self.targets.get_mut(&window_id)?;

        if !target.should_redraw() {
            return None;
        }

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

        if let WindowEventKind::Occluded { occluded } = event.kind {
            target.set_occluded(occluded);
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
