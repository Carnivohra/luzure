use std::error::Error;

use crate::{backend::{BackendError, BackendHandle}, input::InputEvent, window::WindowEvent};

pub trait BackendApplication {
    type Error: Error + From<BackendError>;

    fn started<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), Self::Error>;
    fn resumed<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), Self::Error>;
    fn suspended<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), Self::Error>;
    fn update<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), Self::Error>;
    fn input_event(&mut self, event: InputEvent);
    fn window_event<H: BackendHandle>(&mut self, handle: &mut H, event: WindowEvent) -> Result<(), Self::Error>;
}
