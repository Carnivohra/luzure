use luzure_ecs::Registry;
use luzure_input::input::InputState;

use crate::FrameTime;
use super::MainSchedule;

pub(crate) struct MainRuntime {
    registry: Registry,
    schedule: MainSchedule,
    time: FrameTime,
}

impl MainRuntime {
    pub(crate) fn new() -> Self {
        Self {
            registry: Registry::new(),
            schedule: MainSchedule::new(),
            time: FrameTime::new(),
        }
    }

    pub(crate) const fn registry(&self) -> &Registry {
        &self.registry
    }

    pub(crate) const fn registry_mut(&mut self) -> &mut Registry {
        &mut self.registry
    }

    pub(crate) const fn parts_mut(&mut self) -> (&mut Registry, &mut MainSchedule) {
        (&mut self.registry, &mut self.schedule)
    }

    pub(crate) fn update(&mut self, input: &InputState) {
        self.time.update();
        self.schedule.run(&mut self.registry, input, &self.time);
    }

    pub(crate) fn resume(&mut self) {
        self.time.resume();
    }

    pub(crate) fn suspend(&mut self) {
        self.time.suspend();
    }
}
