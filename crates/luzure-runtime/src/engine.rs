use luzure_backend::{backend::{BackendApplication, BackendHandle}, input::InputEvent, window::{WindowDescriptor, WindowEvent, WindowEventKind}};
use luzure_ecs::Registry;
use luzure_input::input::InputState;
use luzure_render::Renderer;

use crate::{runtime::RuntimeError, window::{PrimaryWindow, WindowManager, WindowState}};

pub struct Engine<R: Renderer> {
    renderer: R,
    _input_state: InputState,
    registry: Registry,
    windows: WindowManager<R::Surface>,
}

impl<R: Renderer> Engine<R> {
    pub fn new(renderer: R) -> Self {
        Self {
            renderer,
            _input_state: InputState::default(),
            registry: Registry::new(),
            windows: WindowManager::new(),
        }
    }
}

impl<R: Renderer> BackendApplication for Engine<R> {
    type Error = RuntimeError;

    fn started<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), Self::Error> {
        let descriptor = WindowDescriptor::default();
        let window = handle.create_window(descriptor.clone())?;
        let window_id = window.id();
        let (width, height) = window.inner_size();
        let entity = self.registry.spawn_empty();
        let surface = self.renderer.create_surface(window.clone(), (width, height))?;

        self.registry.insert(entity, window)?;
        self.registry.insert(entity, WindowState::new(descriptor, width, height))?;
        self.registry.insert(entity, PrimaryWindow)?;
        self.windows.add(window_id, entity, surface);

        Ok(())
    }

    fn resumed<H: BackendHandle>(&mut self, _handle: &mut H) -> Result<(), Self::Error> {
        Ok(())
    }

    fn suspended<H: BackendHandle>(&mut self, _handle: &mut H) -> Result<(), Self::Error> {
        Ok(())
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
