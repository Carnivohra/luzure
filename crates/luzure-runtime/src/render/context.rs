use luzure_render::{MeshDescriptor, MeshHandle};

use crate::{render::{RenderExtractSystem, RenderExtraction, RenderPlan}, runtime::RuntimeError};

pub struct RenderContext<'a> {
    plan: &'a mut RenderPlan,
    render_extraction: &'a mut RenderExtraction,
}

impl<'a> RenderContext<'a> {
    pub(crate) const fn new(plan: &'a mut RenderPlan, render_extraction: &'a mut RenderExtraction) -> Self {
        Self {
            plan,
            render_extraction,
        }
    }

    pub fn create_mesh(&mut self, descriptor: MeshDescriptor) -> Result<MeshHandle, RuntimeError> {
        Ok(self.plan.create(descriptor)?)
    }

    pub fn destroy_mesh(&mut self, mesh: MeshHandle) -> Result<(), RuntimeError> {
        self.plan.destroy(mesh);

        Ok(())
    }

    pub fn add_extract_system(&mut self, system: RenderExtractSystem) {
        self.render_extraction.add_system(system);
    }
}
