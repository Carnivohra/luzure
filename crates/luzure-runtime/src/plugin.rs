mod context;
mod tuple;

pub use context::PluginContext;
pub use crate::{backend::BackendContext, render::RenderContext, simulation::SimulationContext};

use crate::runtime::RuntimeError;

pub trait Plugin {
    fn build(&mut self, context: &mut PluginContext) -> Result<(), RuntimeError>;
}
