use crate::{Window, backend::BackendError, window::WindowDescriptor};

pub trait BackendHandle {
    fn create_window(&mut self, descriptor: WindowDescriptor) -> Result<Window, BackendError>;
    fn destroy_window(&mut self, window: Window) -> Result<(), BackendError>;
    fn exit(&mut self) -> Result<(), BackendError>;
}
