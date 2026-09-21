mod state;

use luzure_render::RenderScene;
use luzure_thread::{TaskRunner, ThreadMode};

use crate::render::{RenderExtraction, RenderSceneTransfer, render_scene_buffer};
use crate::runtime::{RuntimeError, RuntimePlan};

use super::{Simulation, SimulationTask};
use state::SimulationRuntimeState;

pub(crate) struct SimulationRuntime {
    state: Option<SimulationRuntimeState>,
}

impl SimulationRuntime {
    pub(crate) const fn new() -> Self {
        Self {
            state: None,
        }
    }

    pub(crate) fn start(&mut self, simulation: Simulation, render_extraction: RenderExtraction, plan: &RuntimePlan) -> Result<(), RuntimeError> {
        debug_assert!(self.state.is_none());

        let (thread_mode, scene_transfer) = Self::resolve(plan.simulation_thread_mode(), plan.render_scene_transfer())?;

        self.state = Some(match scene_transfer {
            RenderSceneTransfer::Direct => {
                let task = SimulationTask::direct(simulation, render_extraction);
                let runner = TaskRunner::start("luzure-simulation", task, thread_mode, plan.simulation_tick_rate())?;

                SimulationRuntimeState::Direct(runner)
            }
            RenderSceneTransfer::TripleBuffered => {
                let (producer, consumer) = render_scene_buffer();
                let task = SimulationTask::triple_buffered(simulation, render_extraction, producer);
                let runner = TaskRunner::start("luzure-simulation", task, thread_mode, plan.simulation_tick_rate())?;

                SimulationRuntimeState::TripleBuffered {
                    runner,
                    scenes: consumer,
                }
            }
            RenderSceneTransfer::Automatic => unreachable!("automatic render scene transfer must resolve before startup"),
        });

        Ok(())
    }

    pub(crate) fn update(&mut self) -> Result<(), RuntimeError> {
        let Some(state) = &mut self.state else {
            return Ok(());
        };

        state.runner_mut().update()?;
        state.refresh();

        Ok(())
    }

    pub(crate) fn render_scene(&self) -> Option<&RenderScene> {
        self.state.as_ref()?.render_scene()
    }

    pub(crate) fn suspend(&mut self) {
        if let Some(state) = &mut self.state {
            state.runner_mut().pause();
        }
    }

    pub(crate) fn resume(&mut self) {
        if let Some(state) = &mut self.state {
            state.runner_mut().resume();
        }
    }

    pub(crate) fn stop(&mut self) -> Option<RuntimeError> {
        let runner = self.state.take()?.into_runner();

        match runner.stop() {
            Ok((_, Some(error))) => Some(error.into()),
            Ok((_, None)) => None,
            Err(error) => Some(error.into()),
        }
    }

    fn resolve(thread_mode: ThreadMode, scene_transfer: RenderSceneTransfer)
        -> Result<(ThreadMode, RenderSceneTransfer), RuntimeError>
    {
        match (thread_mode, scene_transfer) {
            (ThreadMode::MainThread, RenderSceneTransfer::Direct) => {
                Ok((ThreadMode::MainThread, RenderSceneTransfer::Direct))
            }
            (ThreadMode::Threaded, RenderSceneTransfer::TripleBuffered) => {
                Ok((ThreadMode::Threaded, RenderSceneTransfer::TripleBuffered))
            }
            (ThreadMode::Automatic, RenderSceneTransfer::Direct) => {
                Ok((ThreadMode::MainThread, RenderSceneTransfer::Direct))
            }
            (ThreadMode::Automatic, RenderSceneTransfer::TripleBuffered) => {
                Ok((ThreadMode::Threaded, RenderSceneTransfer::TripleBuffered))
            }
            (ThreadMode::MainThread, RenderSceneTransfer::Automatic) => {
                Ok((ThreadMode::MainThread, RenderSceneTransfer::Direct))
            }
            (ThreadMode::Threaded, RenderSceneTransfer::Automatic) => {
                Ok((ThreadMode::Threaded, RenderSceneTransfer::TripleBuffered))
            }
            (ThreadMode::Automatic, RenderSceneTransfer::Automatic) => {
                #[cfg(target_family = "wasm")]
                {
                    Ok((ThreadMode::MainThread, RenderSceneTransfer::Direct))
                }

                #[cfg(not(target_family = "wasm"))]
                {
                    Ok((ThreadMode::Threaded, RenderSceneTransfer::TripleBuffered))
                }
            }
            _ => Err(RuntimeError::IncompatibleRenderSceneTransfer),
        }
    }
}
