mod startup;
mod system;

pub use startup::{StartupSchedule, StartupSystem};
pub use system::System;

use crate::Registry;

use std::time::Duration;

pub struct Schedule {
    systems: Vec<System>,
}

impl Schedule {
    pub const fn new() -> Self {
        Self { systems: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { systems: Vec::with_capacity(capacity) }
    }

    pub fn add_system(&mut self, system: System) {
        self.systems.push(system);
    }

    pub fn run(&self, registry: &mut Registry, delta: Duration) {
        for system in &self.systems {
            system(registry, delta);
        }
    }
}
