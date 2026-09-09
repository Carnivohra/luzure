use std::time::Duration;

pub trait ThreadTask: Send + 'static {
    fn tick(&mut self, delta: Duration);
}
