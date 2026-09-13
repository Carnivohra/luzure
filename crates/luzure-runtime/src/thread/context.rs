use super::{SimulationThreadPlan, ThreadPlan};

pub struct ThreadContext<'a> {
    plan: &'a mut ThreadPlan,
}

impl<'a> ThreadContext<'a> {
    pub(crate) const fn new(plan: &'a mut ThreadPlan) -> Self {
        Self { plan }
    }

    pub const fn simulation_mut(&mut self) -> &mut SimulationThreadPlan {
        self.plan.simulation_mut()
    }
}
