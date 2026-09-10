use luzure_backend::{backend::{BackendApplication, BackendHandle}, input::InputEvent, window::{WindowEvent, WindowEventKind}};
use luzure_ecs::Registry;
use luzure_game::Game;
use luzure_input::input::InputState;
use luzure_render::{Camera, Renderer};
use luzure_thread::Thread;

use crate::{plugin::{Plugin, PluginContext}, render::{RenderExtraction, RenderSceneConsumer, render_scene_buffer}, runtime::RuntimeError, simulation::{Simulation, SimulationTask}, window::{WindowManager, WindowPlan}};

pub struct Engine<R: Renderer, G: Game<Plugins: Plugin>> {
    renderer: R,
    render_scenes: Option<RenderSceneConsumer>,
    game: G,
    registry: Registry,
    simulation_thread: Option<Thread<SimulationTask>>,
    _input_state: InputState,
    window_plan: WindowPlan,
    windows: WindowManager<R::Surface>,
}

impl<R: Renderer, G: Game<Plugins: Plugin>> Engine<R, G> {
    pub fn new(renderer: R, game: G) -> Self {
        Self {
            renderer,
            render_scenes: None,
            game,
            registry: Registry::new(),
            simulation_thread: None,
            _input_state: InputState::default(),
            window_plan: WindowPlan::new(),
            windows: WindowManager::new(),
        }
    }

    fn start<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), RuntimeError> {
        let (render_scene_producer, render_scene_consumer) = render_scene_buffer();
        let mut render_extraction = RenderExtraction::new();
        let mut simulation = Simulation::new();
        let mut plugins = self.game.plugins();
        let mut context = PluginContext::new(
            G::METADATA,
            &mut self.registry,
            &mut self.window_plan,
            &mut render_extraction,
            &mut simulation,
        );

        plugins.build(&mut context)?;
        self.windows.apply(&mut self.window_plan, &mut self.registry, &mut self.renderer, handle)?;

        let simulation_task = SimulationTask::new(simulation, render_extraction, render_scene_producer);
        let simulation_thread = Thread::spawn("luzure-simulation", simulation_task, Simulation::DEFAULT_TICK_RATE)?;

        self.render_scenes = Some(render_scene_consumer);
        self.simulation_thread = Some(simulation_thread);

        Ok(())
    }

    fn tick(&mut self) -> Result<(), RuntimeError> {
        if let Some(simulation_thread) = &mut self.simulation_thread {
            if let Some(error) = simulation_thread.take_error() {
                return Err(error.into());
            }
        }

        if let Some(render_scenes) = &mut self.render_scenes {
            render_scenes.refresh();
        }

        self.windows.request_redraws(&self.registry);

        Ok(())
    }

    fn stop<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), RuntimeError> {
        let simulation_error = match self.simulation_thread.take() {
            Some(thread) => match thread.stop() {
                Ok((_, Some(error))) => Some(RuntimeError::from(error)),
                Ok((_, None)) => None,
                Err(error) => Some(RuntimeError::from(error)),
            },
            None => None,
        };

        self.render_scenes = None;

        self.windows.destroy_all(&mut self.registry, handle)?;

        if let Some(error) = simulation_error {
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
                self.renderer.resize_surface(surface, (width, height))?;
            }
        }

        if let WindowEventKind::RedrawRequested = event.kind {
            if let Some(surface) = self.windows.surface(event.window_id) {
                if let Some(render_scenes) = &self.render_scenes {
                    let camera = Camera::IDENTITY;
                    let frame = render_scenes.current().frame(&camera);
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
