use luzure_render::{MeshDescriptor, MeshHandle};

use crate::{render::{RenderExtractSystem, RenderExtraction, RenderPlan, RenderSceneTransfer}, runtime::{RuntimeError, RuntimePlan}};

pub struct RenderContext<'a> {
    plan: &'a mut RenderPlan,
    render_extraction: &'a mut RenderExtraction,
    runtime_plan: &'a mut RuntimePlan,
}

impl<'a> RenderContext<'a> {
    pub(crate) const fn new(plan: &'a mut RenderPlan, render_extraction: &'a mut RenderExtraction, runtime_plan: &'a mut RuntimePlan) -> Self {
        Self {
            plan,
            render_extraction,
            runtime_plan,
        }
    }

    pub const fn scene_transfer(&self) -> RenderSceneTransfer {
        self.runtime_plan.render_scene_transfer()
    }

    pub const fn set_scene_transfer(&mut self, scene_transfer: RenderSceneTransfer) {
        self.runtime_plan.set_render_scene_transfer(scene_transfer);
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
