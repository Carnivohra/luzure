use luzure_backend::{backend::{BackendApplication, BackendHandle}, input::InputEvent, window::WindowEvent};
use luzure_input::input::InputState;
use luzure_render::Renderer;

use crate::runtime::RuntimeError;

pub struct Engine<R: Renderer> {
    _renderer: R,
    _input_state: InputState,
}

impl<R: Renderer> Engine<R> {
    pub fn new(_renderer: R) -> Self {
        Self {
            _renderer,
            _input_state: InputState::default(),
        }
    }
}

impl<R: Renderer> BackendApplication for Engine<R> {
    type Error = RuntimeError;

    fn started<H: BackendHandle>(&mut self, _handle: &mut H) -> Result<(), Self::Error> {
        todo!()
    }

    fn resumed<H: BackendHandle>(&mut self, _handle: &mut H) -> Result<(), Self::Error> {
        todo!()
    }

    fn suspended<H: BackendHandle>(&mut self, _handle: &mut H) -> Result<(), Self::Error> {
        todo!()
    }

    fn input_event(&mut self, _event: InputEvent) {
        todo!()
    }

    fn window_event<H: BackendHandle>(&mut self, _handle: &mut H, _event: WindowEvent)
        -> Result<(), Self::Error>
    {
        todo!()
    }
}
