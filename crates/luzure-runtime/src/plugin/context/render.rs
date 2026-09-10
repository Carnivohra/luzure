use luzure_render::{MeshDescriptor, MeshHandle};

use crate::{render::{RenderExtractSystem, RenderExtraction}, runtime::RuntimeError};

pub struct RenderContext<'a> {
    render_extraction: &'a mut RenderExtraction,
}

impl<'a> RenderContext<'a> {
    pub(crate) const fn new(render_extraction: &'a mut RenderExtraction) -> Self {
        Self { render_extraction }
    }

    pub fn create_mesh(&mut self, _descriptor: MeshDescriptor<'_>) -> Result<MeshHandle, RuntimeError> {
        todo!()
    }

    pub fn destroy_mesh(&mut self, _mesh: MeshHandle) -> Result<(), RuntimeError> {
        todo!()
    }

    pub fn add_extract_system(&mut self, system: RenderExtractSystem) {
        self.render_extraction.add_system(system);
    }
}
