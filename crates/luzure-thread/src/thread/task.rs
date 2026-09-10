use std::time::Duration;

pub trait ThreadTask: Send + 'static {
    type Error: Send + 'static;

    fn tick(&mut self, delta: Duration) -> Result<(), Self::Error>;
}

impl<A: ThreadTask, B: ThreadTask<Error = A::Error>> ThreadTask for (A, B) {
    type Error = A::Error;

    fn tick(&mut self, delta: Duration) -> Result<(), Self::Error> {
        self.0.tick(delta)?;
        self.1.tick(delta)
    }
}

impl<A: ThreadTask, B: ThreadTask<Error = A::Error>, C: ThreadTask<Error = A::Error>> ThreadTask for (A, B, C) {
    type Error = A::Error;

    fn tick(&mut self, delta: Duration) -> Result<(), Self::Error> {
        self.0.tick(delta)?;
        self.1.tick(delta)?;
        self.2.tick(delta)
    }
}

impl<A: ThreadTask, B: ThreadTask<Error = A::Error>, C: ThreadTask<Error = A::Error>, D: ThreadTask<Error = A::Error>> ThreadTask for (A, B, C, D) {
    type Error = A::Error;

    fn tick(&mut self, delta: Duration) -> Result<(), Self::Error> {
        self.0.tick(delta)?;
        self.1.tick(delta)?;
        self.2.tick(delta)?;
        self.3.tick(delta)
    }
}
