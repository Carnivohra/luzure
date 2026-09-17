use std::sync::atomic::{AtomicBool, AtomicU32};

use super::error::ThreadErrorSlot;

pub(super) struct ThreadState<E> {
    pub(super) error: ThreadErrorSlot<E>,
    pub(super) paused: AtomicBool,
    pub(super) running: AtomicBool,
    pub(super) tick_rate: AtomicU32,
}

impl<E> ThreadState<E> {
    pub(super) const fn new(tick_rate: u32) -> Self {
        Self {
            error: ThreadErrorSlot::new(),
            paused: AtomicBool::new(false),
            running: AtomicBool::new(true),
            tick_rate: AtomicU32::new(tick_rate),
        }
    }
}
