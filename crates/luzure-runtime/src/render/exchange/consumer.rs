use luzure_render::RenderScene;

use std::sync::Arc;

use super::inner::RenderSceneBufferInner;

pub(crate) struct RenderSceneConsumer {
    inner: Arc<RenderSceneBufferInner>,
    scene: usize,
}

impl RenderSceneConsumer {
    pub(super) fn new(inner: Arc<RenderSceneBufferInner>, scene: usize) -> Self {
        Self {
            inner,
            scene,
        }
    }

    pub(crate) fn refresh(&mut self) -> bool {
        let Some(scene) = self.inner.state().take(self.scene) else {
            return false;
        };

        self.scene = scene;
        true
    }

    pub(crate) fn current(&self) -> &RenderScene {
        unsafe { &*self.inner.scene(self.scene) }
    }
}
