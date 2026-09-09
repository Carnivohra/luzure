use std::sync::atomic::{AtomicUsize, Ordering};

#[repr(align(64))]
pub(super) struct RenderExchangeState {
    value: AtomicUsize,
}

impl RenderExchangeState {
    const INDEX_MASK: usize = 0b11;
    const READY: usize = 0b100;

    pub(super) fn new(middle_scene: usize) -> Self {
        debug_assert!(middle_scene < 3);

        Self {
            value: AtomicUsize::new(middle_scene),
        }
    }

    pub(super) fn publish(&self, scene: usize) -> usize {
        debug_assert!(scene < 3);

        self.value.swap(scene | Self::READY, Ordering::AcqRel) & Self::INDEX_MASK
    }

    pub(super) fn take(&self, scene: usize) -> Option<usize> {
        debug_assert!(scene < 3);

        let state = self.value.load(Ordering::Acquire);

        if state & Self::READY == 0 {
            return None;
        }

        self.value.compare_exchange(
            state,
            scene,
            Ordering::AcqRel,
            Ordering::Acquire,
        ).ok()
            .map(|state| state & Self::INDEX_MASK)
    }
}
