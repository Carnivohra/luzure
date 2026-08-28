use luzure_backend::{backend::{BackendApplication, BackendHandle}, input::InputEvent, window::{WindowDescriptor, WindowEvent, WindowEventKind}};
use luzure_ecs::{Entity, Registry};
use luzure_input::input::InputState;
use luzure_render::Renderer;

use crate::{runtime::RuntimeError, window::WindowManager};

pub struct Engine<R: Renderer> {
    renderer: R,
    _input_state: InputState,
    registry: Registry,
    windows: WindowManager<R::Surface>,
    primary_window: Option<Entity>,
}

impl<R: Renderer> Engine<R> {
    pub fn new(renderer: R) -> Self {
        Self {
            renderer,
            _input_state: InputState::default(),
            registry: Registry::new(),
            windows: WindowManager::new(),
            primary_window: None,
        }
    }

    fn start<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), RuntimeError> {
        let entity = self.windows.create(&mut self.registry, &mut self.renderer, handle, WindowDescriptor::default())?;
        self.primary_window = Some(entity);

        Ok(())
    }

    fn tick(&mut self) -> Result<(), RuntimeError> {
        self.windows.request_redraws(&self.registry);

        Ok(())
    }
}

impl<R: Renderer> BackendApplication for Engine<R> {
    type Error = RuntimeError;

    fn started<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), Self::Error> {
        self.start(handle)
    }

    fn resumed<H: BackendHandle>(&mut self, _handle: &mut H) -> Result<(), Self::Error> {
        Ok(())
    }

    fn suspended<H: BackendHandle>(&mut self, _handle: &mut H) -> Result<(), Self::Error> {
        Ok(())
    }

    fn about_to_wait<H: BackendHandle>(&mut self, _handle: &mut H) -> Result<(), Self::Error> {
        self.tick()
    }

    fn input_event(&mut self, _event: InputEvent) {
        todo!()
    }

    fn window_event<H: BackendHandle>(&mut self, _handle: &mut H, event: WindowEvent)
        -> Result<(), Self::Error>
    {
        self.windows.synchronize(&mut self.registry, event);

        if let WindowEventKind::RedrawRequested = event.kind {
            if let Some(surface) = self.windows.surface(event.window_id) {
                self.renderer.render(surface)?;
            }
        }

        Ok(())
    }
}
