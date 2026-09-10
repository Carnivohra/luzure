mod context;
mod tuple;

pub use context::{BackendContext, PluginContext, RenderContext, SimulationContext};

use crate::runtime::RuntimeError;

pub trait Plugin {
    fn build(&mut self, context: &mut PluginContext) -> Result<(), RuntimeError>;
}
