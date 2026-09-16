use luzure_ecs::Registry;
use luzure_input::input::InputState;

use crate::FrameTime;
use super::MainSystem;

pub(crate) struct MainSchedule {
    systems: Vec<MainSystem>,
}

impl MainSchedule {
    pub(crate) const fn new() -> Self {
        Self { systems: Vec::new() }
    }

    pub(crate) fn add_system(&mut self, system: MainSystem) {
        self.systems.push(system);
    }

    pub(crate) fn run(&self, registry: &mut Registry, input: &InputState, time: &FrameTime) {
        for system in &self.systems {
            system(registry, input, time);
        }
    }
}
