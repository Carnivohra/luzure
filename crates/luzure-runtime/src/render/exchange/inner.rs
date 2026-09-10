use luzure_render::RenderScene;

use std::cell::UnsafeCell;

use super::state::RenderSceneBufferState;

pub(super) struct RenderSceneBufferInner {
    state: RenderSceneBufferState,
    scenes: [UnsafeCell<RenderScene>; 3],
}

impl RenderSceneBufferInner {
    pub(super) fn new(middle_scene: usize) -> Self {
        Self {
            state: RenderSceneBufferState::new(middle_scene),
            scenes: [
                UnsafeCell::new(RenderScene::new()),
                UnsafeCell::new(RenderScene::new()),
                UnsafeCell::new(RenderScene::new()),
            ],
        }
    }

    pub(super) const fn state(&self) -> &RenderSceneBufferState {
        &self.state
    }

    pub(super) fn scene(&self, index: usize) -> *mut RenderScene {
        self.scenes[index].get()
    }
}

unsafe impl Sync for RenderSceneBufferInner {}
