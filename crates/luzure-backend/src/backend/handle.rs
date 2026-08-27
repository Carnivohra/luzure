use crate::{Window, backend::BackendError, window::{WindowDescriptor, WindowId}};

pub trait BackendHandle {
    fn create_window(&mut self, descriptor: WindowDescriptor) -> Result<Window, BackendError>;
    fn destroy_window(&mut self, window_id: WindowId) -> Result<(), BackendError>;
    fn exit(&mut self) -> Result<(), BackendError>;
}
