use super::{Plugin, PluginContext};

use crate::runtime::RuntimeError;

impl Plugin for () {
    fn build(&mut self, _context: &mut PluginContext) -> Result<(), RuntimeError> {
        Ok(())
    }
}

impl<A: Plugin, B: Plugin> Plugin for (A, B) {
    fn build(&mut self, context: &mut PluginContext) -> Result<(), RuntimeError> {
        self.0.build(context)?;
        self.1.build(context)
    }
}

impl<A: Plugin, B: Plugin, C: Plugin> Plugin for (A, B, C) {
    fn build(&mut self, context: &mut PluginContext) -> Result<(), RuntimeError> {
        self.0.build(context)?;
        self.1.build(context)?;
        self.2.build(context)
    }
}

impl<A: Plugin, B: Plugin, C: Plugin, D: Plugin> Plugin for (A, B, C, D) {
    fn build(&mut self, context: &mut PluginContext) -> Result<(), RuntimeError> {
        self.0.build(context)?;
        self.1.build(context)?;
        self.2.build(context)?;
        self.3.build(context)
    }
}
