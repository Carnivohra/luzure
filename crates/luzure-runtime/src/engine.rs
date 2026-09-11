use luzure_backend::{backend::{BackendApplication, BackendHandle}, input::InputEvent, window::{WindowEvent, WindowEventKind}};
use luzure_ecs::Registry;
use luzure_game::Game;
use luzure_input::input::InputState;
use luzure_render::{Camera, Renderer};

use crate::{plugin::{Plugin, PluginContext}, render::{RenderExtraction, RenderRuntime}, runtime::RuntimeError, simulation::{Simulation, SimulationTask}, thread::ThreadManager, window::WindowManager};

pub struct Engine<R: Renderer, G: Game<Plugins: Plugin>> {
    render: RenderRuntime<R>,
    threads: ThreadManager,
    game: G,
    registry: Registry,
    _input_state: InputState,
    windows: WindowManager<R::Surface>,
}

impl<R: Renderer, G: Game<Plugins: Plugin>> Engine<R, G> {
    pub fn new(renderer: R, game: G) -> Self {
        Self {
            render: RenderRuntime::new(renderer),
            threads: ThreadManager::new(),
            game,
            registry: Registry::new(),
            _input_state: InputState::default(),
            windows: WindowManager::new(),
        }
    }

    fn start<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), RuntimeError> {
        let mut render_extraction = RenderExtraction::new();
        let mut simulation = Simulation::new();
        let mut plugins = self.game.plugins();
        {
            let mut context = PluginContext::new(
                G::METADATA,
                &mut self.registry,
                self.windows.plan_mut(),
                &mut render_extraction,
                &mut simulation,
            );

            plugins.build(&mut context)?;
        }

        self.windows.apply(&mut self.registry, self.render.renderer_mut(), handle)?;

        let render_scene_producer = self.render.start();
        let simulation_task = SimulationTask::new(simulation, render_extraction, render_scene_producer);

        self.threads.start_simulation(simulation_task, Simulation::DEFAULT_TICK_RATE)?;

        Ok(())
    }

    fn tick(&mut self) -> Result<(), RuntimeError> {
        if let Some(error) = self.threads.take_error() {
            return Err(error);
        }

        self.render.update();

        self.windows.request_redraws(&self.registry);

        Ok(())
    }

    fn stop<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), RuntimeError> {
        let thread_error = self.threads.stop();

        self.render.stop();

        self.windows.destroy_all(&mut self.registry, handle)?;

        if let Some(error) = thread_error {
            return Err(error);
        }

        Ok(())
    }
}

impl<R: Renderer, G: Game<Plugins: Plugin>> BackendApplication for Engine<R, G> {
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
                self.render.resize_surface(surface, (width, height))?;
            }
        }

        if let WindowEventKind::RedrawRequested = event.kind {
            if let Some(surface) = self.windows.surface(event.window_id) {
                let camera = Camera::IDENTITY;

                self.render.render(surface, &camera)?;
            }
        }

        Ok(())
    }

    fn stopped<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), Self::Error> {
        self.stop(handle)
    }
}
