use std::rc::Rc;

use luzure_backend::{backend::BackendError, window::WindowId};
use winit::window::WindowId as WinitWindowId;

use super::{WinitWindow, entry::WinitWindowEntry};

pub(crate) struct WinitWindowRegistry {
    next_id: u64,
    windows: Vec<WinitWindowEntry>,
}

impl WinitWindowRegistry {
    pub(crate) const fn new() -> Self {
        Self {
            next_id: 0,
            windows: vec![],
        }
    }

    pub(crate) fn insert(&mut self, window: Rc<WinitWindow>) -> Result<WindowId, BackendError> {
        let next_id = self.next_id.checked_add(1)
            .ok_or(BackendError::WindowCreation)?;
        let window_id = WindowId::new(self.next_id);

        self.windows.push(WinitWindowEntry::new(window_id, window));
        self.next_id = next_id;

        Ok(window_id)
    }

    pub(crate) fn window_id(&self, winit_id: WinitWindowId) -> Option<WindowId> {
        self.windows.iter()
            .find(|window| window.winit_id() == winit_id)
            .map(WinitWindowEntry::window_id)
    }

    pub(crate) fn remove(&mut self, window_id: WindowId) -> Result<(), BackendError> {
        let index = self.windows.iter()
            .position(|window| window.window_id() == window_id)
            .ok_or(BackendError::InvalidWindow)?;

        self.windows.swap_remove(index);

        Ok(())
    }

    pub(crate) fn clear(&mut self) {
        self.windows.clear();
    }
}
