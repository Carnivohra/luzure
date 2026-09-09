use luzure_render::RenderScene;

use std::cell::UnsafeCell;

use super::state::RenderExchangeState;

pub(super) struct RenderExchangeInner {
    state: RenderExchangeState,
    scenes: [UnsafeCell<RenderScene>; 3],
}

impl RenderExchangeInner {
    pub(super) fn new(middle_scene: usize) -> Self {
        Self {
            state: RenderExchangeState::new(middle_scene),
            scenes: [
                UnsafeCell::new(RenderScene::new()),
                UnsafeCell::new(RenderScene::new()),
                UnsafeCell::new(RenderScene::new()),
            ],
        }
    }

    pub(super) const fn state(&self) -> &RenderExchangeState {
        &self.state
    }

    pub(super) fn scene(&self, index: usize) -> *mut RenderScene {
        self.scenes[index].get()
    }
}

unsafe impl Sync for RenderExchangeInner {}
