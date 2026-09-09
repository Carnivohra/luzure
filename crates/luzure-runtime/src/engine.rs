use luzure_backend::{backend::{BackendApplication, BackendHandle}, input::InputEvent, window::{WindowDescriptor, WindowEvent, WindowEventKind}};
use luzure_ecs::Registry;
use luzure_game::Game;
use luzure_input::input::InputState;
use luzure_render::{Camera, Renderer};
use luzure_thread::Thread;

use crate::{render::{RenderExchange, RenderReader}, runtime::RuntimeError, simulation::Simulation, window::{PrimaryWindow, WindowManager}};

pub struct Engine<R: Renderer, G: Game> {
    renderer: R,
    render_reader: Option<RenderReader>,
    game: G,
    registry: Registry,
    simulation_thread: Option<Thread<Simulation>>,
    _input_state: InputState,
    windows: WindowManager<R::Surface>,
}

impl<R: Renderer, G: Game> Engine<R, G> {
    pub fn new(renderer: R, game: G) -> Self {
        Self {
            renderer,
            render_reader: None,
            game,
            registry: Registry::new(),
            simulation_thread: None,
            _input_state: InputState::default(),
            windows: WindowManager::new(),
        }
    }

    fn start<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), RuntimeError> {
        let exchange = RenderExchange::new();
        let (render_reader, render_writer) = exchange.split();
        let simulation = Simulation::new(render_writer);
        let simulation_thread = Thread::spawn("luzure-simulation", simulation, Simulation::DEFAULT_TICK_RATE)?;

        self.render_reader = Some(render_reader);
        self.simulation_thread = Some(simulation_thread);

        let descriptor = WindowDescriptor {
            title: self.game.metadata().title.to_owned(),
            ..Default::default()
        };

        let entity = self.windows.create(&mut self.registry, &mut self.renderer, handle, descriptor)?;
        self.registry.insert(entity, PrimaryWindow)?;

        Ok(())
    }

    fn tick(&mut self) -> Result<(), RuntimeError> {
        if let Some(render_reader) = &mut self.render_reader {
            render_reader.update();
        }

        self.windows.request_redraws(&self.registry);

        Ok(())
    }

    fn stop<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), RuntimeError> {
        if let Some(thread) = self.simulation_thread.take() {
            thread.stop()?;
        }

        self.render_reader = None;

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
                if let Some(render_reader) = &self.render_reader {
                    let camera = Camera::IDENTITY;
                    let frame = render_reader.scene().frame(&camera);
                    self.renderer.render(surface, &frame)?;
                }
            }
        }

        Ok(())
    }

    fn stopped<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), Self::Error> {
        self.stop(handle)
    }
}
