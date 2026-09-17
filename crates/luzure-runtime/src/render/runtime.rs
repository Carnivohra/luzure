use luzure_ecs::Registry;
use luzure_render::{RenderTarget, RenderView, Renderer, RendererStatus, render::RenderError};

use super::{RenderPlan, RenderSceneConsumer, RenderSceneProducer, extract_cameras, render_scene_buffer};

pub(crate) struct RenderRuntime<R: Renderer> {
    plan: RenderPlan,
    renderer: R,
    scenes: Option<RenderSceneConsumer>,
    views: Vec<RenderView>,
}

impl<R: Renderer> RenderRuntime<R> {
    pub(crate) const fn new(renderer: R) -> Self {
        Self {
            plan: RenderPlan::new(),
            renderer,
            scenes: None,
            views: Vec::new(),
        }
    }

    pub(crate) const fn plan_mut(&mut self) -> &mut RenderPlan {
        &mut self.plan
    }

    pub(crate) fn start(&mut self) -> RenderSceneProducer {
        debug_assert!(self.scenes.is_none());

        let (producer, consumer) = render_scene_buffer();

        self.scenes = Some(consumer);
        producer
    }

    pub(crate) fn update(&mut self, registry: &Registry) -> Result<(), RenderError> {
        if self.renderer.update()? == RendererStatus::Ready {
            self.plan.apply(&mut self.renderer)?;
        }

        if let Some(scenes) = &mut self.scenes {
            scenes.refresh();
        }

        extract_cameras(registry, &mut self.views);

        Ok(())
    }

    pub(crate) fn render(&mut self, surface: &mut R::Surface, target: RenderTarget)
        -> Result<(), RenderError>
    {
        let Some(scenes) = &self.scenes else {
            return Ok(());
        };

        let target = target.value();
        let first = self.views.partition_point(|view| view.target().value() < target);
        let count = self.views[first..].partition_point(|view| view.target().value() == target);

        let scene = scenes.current();
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
        self.scenes = None;
    }
}
