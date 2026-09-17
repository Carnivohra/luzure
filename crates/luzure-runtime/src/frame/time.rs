use std::time::Duration;

#[cfg(not(target_family = "wasm"))]
use std::time::Instant;

#[cfg(target_family = "wasm")]
use web_time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct FrameTime {
    delta: Duration,
    elapsed: Duration,
    frame: u64,
    last_update: Option<Instant>,
}

impl FrameTime {
    pub(crate) const fn new() -> Self {
        Self {
            delta: Duration::ZERO,
            elapsed: Duration::ZERO,
            frame: 0,
            last_update: None,
        }
    }

    pub const fn delta(&self) -> Duration {
        self.delta
    }

    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub fn elapsed_seconds(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }

    pub const fn frame(&self) -> u64 {
        self.frame
    }

    pub(crate) fn update(&mut self) {
        let now = Instant::now();

        self.delta = self.last_update
            .map_or(Duration::ZERO, |last_update| now.saturating_duration_since(last_update));
        self.elapsed = self.elapsed.saturating_add(self.delta);
        self.frame = self.frame.saturating_add(1);
        self.last_update = Some(now);
    }

    pub(crate) fn resume(&mut self) {
        self.delta = Duration::ZERO;
        self.last_update = Some(Instant::now());
    }

    pub(crate) fn suspend(&mut self) {
        self.delta = Duration::ZERO;
        self.last_update = None;
    }
}
