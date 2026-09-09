use luzure_render::RenderScene;

use std::sync::Arc;

use super::inner::RenderExchangeInner;

pub(crate) struct RenderReader {
    inner: Arc<RenderExchangeInner>,
    scene: usize,
}

impl RenderReader {
    pub(super) fn new(inner: Arc<RenderExchangeInner>, scene: usize) -> Self {
        Self {
            inner,
            scene,
        }
    }

    pub(crate) fn update(&mut self) -> bool {
        let Some(scene) = self.inner.state().take(self.scene) else {
            return false;
        };

        self.scene = scene;
        true
    }

    pub(crate) fn scene(&self) -> &RenderScene {
        unsafe { &*self.inner.scene(self.scene) }
    }
}
