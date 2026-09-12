use luzure_render::{Camera, Renderer, render::RenderError};

use super::{RenderPlan, RenderSceneConsumer, RenderSceneProducer, render_scene_buffer};

pub(crate) struct RenderRuntime<R: Renderer> {
    plan: RenderPlan,
    renderer: R,
    scenes: Option<RenderSceneConsumer>,
}

impl<R: Renderer> RenderRuntime<R> {
    pub(crate) const fn new(renderer: R) -> Self {
        Self {
            plan: RenderPlan::new(),
            renderer,
            scenes: None,
        }
    }

    pub(crate) const fn plan_mut(&mut self) -> &mut RenderPlan {
        &mut self.plan
    }

    pub(crate) fn apply(&mut self) -> Result<(), RenderError> {
        self.plan.apply(&mut self.renderer)
    }

    pub(crate) fn start(&mut self) -> RenderSceneProducer {
        debug_assert!(self.scenes.is_none());

        let (producer, consumer) = render_scene_buffer();

        self.scenes = Some(consumer);
        producer
    }

    pub(crate) fn update(&mut self) -> bool {
        self.scenes.as_mut()
            .is_some_and(RenderSceneConsumer::refresh)
    }

    pub(crate) fn resize_surface(&mut self, surface: &mut R::Surface, size: (u32, u32))
        -> Result<(), RenderError>
    {
        self.renderer.resize_surface(surface, size)
    }

    pub(crate) fn render(&mut self, surface: &R::Surface, camera: &Camera)
        -> Result<(), RenderError>
    {
        let Some(scenes) = &self.scenes else {
            return Ok(());
        };

        let frame = scenes.current().frame(camera);

        self.renderer.render(surface, &frame)
    }

    pub(crate) const fn renderer_mut(&mut self) -> &mut R {
        &mut self.renderer
    }

    pub(crate) fn stop(&mut self) {
        self.scenes = None;
    }
}
