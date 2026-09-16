use luzure_ecs::Entity;
use luzure_render::RenderTarget;

pub(crate) const fn render_target(window: Entity) -> RenderTarget {
    RenderTarget::new((window.generation() as u64) << 32 | window.index() as u64)
}
