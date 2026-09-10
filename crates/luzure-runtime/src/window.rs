mod manager;
mod plan;
mod primary;
mod state;

pub use manager::WindowManager;
pub use primary::PrimaryWindow;
pub use state::WindowState;
pub(crate) use plan::WindowPlan;
