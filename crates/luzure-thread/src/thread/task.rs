use std::time::Duration;

pub trait ThreadTask: Send + 'static {
    fn tick(&mut self, delta: Duration);
}

impl<A: ThreadTask, B: ThreadTask> ThreadTask for (A, B) {
    fn tick(&mut self, delta: Duration) {
        self.0.tick(delta);
        self.1.tick(delta);
    }
}

impl<A: ThreadTask, B: ThreadTask, C: ThreadTask> ThreadTask for (A, B, C) {
    fn tick(&mut self, delta: Duration) {
        self.0.tick(delta);
        self.1.tick(delta);
        self.2.tick(delta);
    }
}

impl<A: ThreadTask, B: ThreadTask, C: ThreadTask, D: ThreadTask> ThreadTask for (A, B, C, D) {
    fn tick(&mut self, delta: Duration) {
        self.0.tick(delta);
        self.1.tick(delta);
        self.2.tick(delta);
        self.3.tick(delta);
    }
}
