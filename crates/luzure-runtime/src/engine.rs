use luzure_backend::{backend::{BackendApplication, BackendHandle}, input::InputEvent, window::{WindowDescriptor, WindowEvent, WindowEventKind}};
use luzure_ecs::{Entity, Registry};
use luzure_game::Game;
use luzure_input::input::InputState;
use luzure_render::{Camera, RenderFrame, Renderer};

use crate::{runtime::RuntimeError, window::WindowManager};

pub struct Engine<R: Renderer, G: Game> {
    renderer: R,
    game: G,
    render_camera: Camera,
    _input_state: InputState,
    registry: Registry,
    windows: WindowManager<R::Surface>,
    primary_window: Option<Entity>,
}

impl<R: Renderer, G: Game> Engine<R, G> {
    pub fn new(renderer: R, game: G) -> Self {
        Self {
            renderer,
            game,
            render_camera: Camera::IDENTITY,
            _input_state: InputState::default(),
            registry: Registry::new(),
            windows: WindowManager::new(),
            primary_window: None,
        }
    }

    fn start<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), RuntimeError> {
        let descriptor = WindowDescriptor {
            title: self.game.metadata().title.to_owned(),
            ..Default::default()
        };

        let entity = self.windows.create(&mut self.registry, &mut self.renderer, handle, descriptor)?;
        self.primary_window = Some(entity);

        Ok(())
    }

    fn tick(&mut self) -> Result<(), RuntimeError> {
        self.windows.request_redraws(&self.registry);

        Ok(())
    }

    fn stop<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), RuntimeError> {
        self.primary_window = None;
        self.windows.destroy_all(&mut self.registry, handle)?;

        Ok(())
    }
}

impl<R: Renderer, G: Game> BackendApplication for Engine<R, G> {
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

    fn update<H: BackendHandle>(&mut self, _handle: &mut H) -> Result<(), Self::Error> {
        self.tick()
    }

    fn input_event(&mut self, _event: InputEvent) {
        todo!()
    }

    fn window_event<H: BackendHandle>(&mut self, _handle: &mut H, event: WindowEvent)
        -> Result<(), Self::Error>
    {
        self.windows.synchronize(&mut self.registry, event);

        if let WindowEventKind::Resized { width, height } = event.kind {
            if let Some(surface) = self.windows.surface_mut(event.window_id) {
                self.renderer.resize_surface(surface, (width, height))?;
            }
        }

        if let WindowEventKind::RedrawRequested = event.kind {
            if let Some(surface) = self.windows.surface(event.window_id) {
                let frame = RenderFrame::new(&self.render_camera);
                self.renderer.render(surface, &frame)?;
            }
        }

        Ok(())
    }

    fn stopped<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), Self::Error> {
        self.stop(handle)
    }
}
