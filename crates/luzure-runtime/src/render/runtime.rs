use luzure_ecs::Registry;
use luzure_render::{RenderScene, RenderTarget, RenderView, Renderer, RendererStatus, render::RenderError};

use super::{RenderPlan, extract_cameras};

pub(crate) struct RenderRuntime<R: Renderer> {
    plan: RenderPlan,
    renderer: R,
    views: Vec<RenderView>,
}

impl<R: Renderer> RenderRuntime<R> {
    pub(crate) const fn new(renderer: R) -> Self {
        Self {
            plan: RenderPlan::new(),
            renderer,
            views: Vec::new(),
        }
    }

    pub(crate) const fn plan_mut(&mut self) -> &mut RenderPlan {
        &mut self.plan
    }

    pub(crate) fn update(&mut self, registry: &Registry) -> Result<(), RenderError> {
        if self.renderer.update()? == RendererStatus::Ready {
            self.plan.apply(&mut self.renderer)?;
        }

        extract_cameras(registry, &mut self.views);

        Ok(())
    }

    pub(crate) fn render(&mut self, surface: &mut R::Surface, target: RenderTarget, scene: &RenderScene)
        -> Result<(), RenderError>
    {
        let target = target.value();
        let first = self.views.partition_point(|view| view.target().value() < target);
        let count = self.views[first..].partition_point(|view| view.target().value() == target);

        let frame = scene.frame(&self.views[first..first + count]);

        self.renderer.render(surface, &frame)
    }

    pub(crate) const fn renderer_mut(&mut self) -> &mut R {
        &mut self.renderer
    }

    pub(crate) fn suspend(&mut self) {
        self.renderer.suspend();
    }

    pub(crate) fn stop(&mut self) {
        self.suspend();
    }
}
