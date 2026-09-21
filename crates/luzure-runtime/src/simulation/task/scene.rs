use luzure_render::RenderScene;

use crate::render::RenderSceneProducer;

pub(super) enum SimulationTaskScene {
    Direct(RenderScene),
    TripleBuffered(RenderSceneProducer),
}

impl SimulationTaskScene {
    pub(super) const fn direct() -> Self {
        Self::Direct(RenderScene::new())
    }

    pub(super) const fn triple_buffered(producer: RenderSceneProducer) -> Self {
        Self::TripleBuffered(producer)
    }

    pub(super) const fn current(&self) -> Option<&RenderScene> {
        match self {
            Self::Direct(scene) => Some(scene),
            Self::TripleBuffered(_) => None,
        }
    }

    pub(super) fn publish<E, F: FnOnce(&mut RenderScene) -> Result<(), E>>(&mut self, write: F)
        -> Result<(), E>
    {
        match self {
            Self::Direct(scene) => write(scene),
            Self::TripleBuffered(producer) => producer.publish(write),
        }
    }
}
