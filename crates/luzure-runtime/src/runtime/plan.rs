use luzure_thread::{ThreadError, ThreadMode};

use crate::render::RenderSceneTransfer;
use crate::simulation::Simulation;

pub(crate) struct RuntimePlan {
    render_scene_transfer: RenderSceneTransfer,
    simulation_thread_mode: ThreadMode,
    simulation_tick_rate: u32,
}

impl RuntimePlan {
    pub(crate) const fn new() -> Self {
        Self {
            render_scene_transfer: RenderSceneTransfer::Automatic,
            simulation_thread_mode: ThreadMode::Automatic,
            simulation_tick_rate: Simulation::DEFAULT_TICK_RATE,
        }
    }

    pub(crate) const fn render_scene_transfer(&self) -> RenderSceneTransfer {
        self.render_scene_transfer
    }

    pub(crate) const fn set_render_scene_transfer(&mut self, render_scene_transfer: RenderSceneTransfer) {
        self.render_scene_transfer = render_scene_transfer;
    }

    pub(crate) const fn simulation_thread_mode(&self) -> ThreadMode {
        self.simulation_thread_mode
    }

    pub(crate) const fn set_simulation_thread_mode(&mut self, simulation_thread_mode: ThreadMode) {
        self.simulation_thread_mode = simulation_thread_mode;
    }

    pub(crate) const fn simulation_tick_rate(&self) -> u32 {
        self.simulation_tick_rate
    }

    pub(crate) fn set_simulation_tick_rate(&mut self, simulation_tick_rate: u32) -> Result<(), ThreadError> {
        if simulation_tick_rate == 0 {
            return Err(ThreadError::InvalidTickRate);
        }

        self.simulation_tick_rate = simulation_tick_rate;

        Ok(())
    }
}
