use std::sync::{Mutex, atomic::{AtomicBool, Ordering}};

pub(crate) struct ThreadErrorSlot<E> {
    error: Mutex<Option<E>>,
    ready: AtomicBool,
}

impl<E> ThreadErrorSlot<E> {
    pub(crate) const fn new() -> Self {
        Self {
            error: Mutex::new(None),
            ready: AtomicBool::new(false),
        }
    }

    pub(crate) fn store(&self, error: E) {
        let mut slot = match self.error.lock() {
            Ok(slot) => slot,
            Err(error) => error.into_inner(),
        };

        *slot = Some(error);
        self.ready.store(true, Ordering::Release);
    }

    pub(crate) fn take(&self) -> Option<E> {
        if !self.ready.load(Ordering::Acquire) {
            return None;
        }

        let mut slot = match self.error.lock() {
            Ok(slot) => slot,
            Err(error) => error.into_inner(),
        };
        let error = slot.take();

        self.ready.store(false, Ordering::Release);
        error
    }
}
