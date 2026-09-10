mod consumer;
mod inner;
mod producer;
mod state;

pub(crate) use consumer::RenderSceneConsumer;
pub(crate) use producer::RenderSceneProducer;

use inner::RenderSceneBufferInner;

use std::sync::Arc;

pub(crate) fn render_scene_buffer() -> (RenderSceneProducer, RenderSceneConsumer) {
    let inner = Arc::new(RenderSceneBufferInner::new(1));
    let producer = RenderSceneProducer::new(Arc::clone(&inner), 2);
    let consumer = RenderSceneConsumer::new(inner, 0);

    (producer, consumer)
}
