use luzure_render::RenderScene;
use luzure_thread::TaskRunner;

use crate::render::RenderSceneConsumer;

use super::super::SimulationTask;

pub(super) enum SimulationRuntimeState {
    Direct(TaskRunner<SimulationTask>),
    TripleBuffered {
        runner: TaskRunner<SimulationTask>,
        scenes: RenderSceneConsumer,
    },
}

impl SimulationRuntimeState {
    pub(super) fn runner_mut(&mut self) -> &mut TaskRunner<SimulationTask> {
        match self {
            Self::Direct(runner) => runner,
            Self::TripleBuffered { runner, .. } => runner,
        }
    }

    pub(super) fn refresh(&mut self) {
        if let Self::TripleBuffered { scenes, .. } = self {
            scenes.refresh();
        }
    }

    pub(super) fn render_scene(&self) -> Option<&RenderScene> {
        match self {
            Self::Direct(runner) => runner.main_thread_task().and_then(SimulationTask::render_scene),
            Self::TripleBuffered { scenes, .. } => Some(scenes.current()),
        }
    }

    pub(super) fn into_runner(self) -> TaskRunner<SimulationTask> {
        match self {
            Self::Direct(runner) => runner,
            Self::TripleBuffered { runner, .. } => runner,
        }
    }
}
