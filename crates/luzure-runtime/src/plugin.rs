mod context;
mod tuple;

pub use context::PluginContext;

use crate::runtime::RuntimeError;

pub trait Plugin {
    fn build(&mut self, context: &mut PluginContext) -> Result<(), RuntimeError>;
}
