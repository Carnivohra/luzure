mod inner;
mod reader;
mod state;
mod writer;

pub(crate) use reader::RenderReader;
pub(crate) use writer::RenderWriter;

use inner::RenderExchangeInner;

use std::sync::Arc;

pub(crate) struct RenderExchange {
    inner: Arc<RenderExchangeInner>,
}

impl RenderExchange {
    const READER_SCENE: usize = 0;
    const MIDDLE_SCENE: usize = 1;
    const WRITER_SCENE: usize = 2;

    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(RenderExchangeInner::new(Self::MIDDLE_SCENE)),
        }
    }

    pub(crate) fn split(self) -> (RenderReader, RenderWriter) {
        let reader = RenderReader::new(Arc::clone(&self.inner), Self::READER_SCENE);
        let writer = RenderWriter::new(self.inner, Self::WRITER_SCENE);

        (reader, writer)
    }
}
