use luzure_render::RenderScene;

use std::sync::Arc;

use super::inner::RenderSceneBufferInner;

pub(crate) struct RenderSceneProducer {
    inner: Arc<RenderSceneBufferInner>,
    scene: usize,
}

impl RenderSceneProducer {
    pub(super) fn new(inner: Arc<RenderSceneBufferInner>, scene: usize) -> Self {
        Self {
            inner,
            scene,
        }
    }

    pub(crate) fn publish<E, F: FnOnce(&mut RenderScene) -> Result<(), E>>(&mut self, write: F)
        -> Result<(), E>
    {
        write(unsafe { &mut *self.inner.scene(self.scene) })?;
        self.scene = self.inner.state().publish(self.scene);

        Ok(())
    }
}
