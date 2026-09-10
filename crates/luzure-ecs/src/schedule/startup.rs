mod system;

pub use system::StartupSystem;

use crate::Registry;

pub struct StartupSchedule {
    systems: Vec<StartupSystem>,
}

impl StartupSchedule {
    pub const fn new() -> Self {
        Self { systems: Vec::new() }
    }

    pub fn add_system(&mut self, system: StartupSystem) {
        self.systems.push(system);
    }

    pub fn run(&self, registry: &mut Registry) {
        for system in &self.systems {
            system(registry);
        }
    }
}
