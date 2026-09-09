use luzure_render::RenderScene;

use std::sync::Arc;

use super::inner::RenderExchangeInner;

pub(crate) struct RenderWriter {
    inner: Arc<RenderExchangeInner>,
    scene: usize,
}

impl RenderWriter {
    pub(super) fn new(inner: Arc<RenderExchangeInner>, scene: usize) -> Self {
        Self {
            inner,
            scene,
        }
    }

    pub(crate) fn scene_mut(&mut self) -> &mut RenderScene {
        unsafe { &mut *self.inner.scene(self.scene) }
    }

    pub(crate) fn publish(&mut self) {
        self.scene = self.inner.state().publish(self.scene);
    }
}
