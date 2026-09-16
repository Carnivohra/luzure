use luzure_backend::{backend::{BackendApplication, BackendHandle}, input::InputEvent, window::{WindowEvent, WindowEventKind}};
use luzure_game::Game;
use luzure_render::Renderer;

use crate::{input::InputRuntime, main_thread::MainRuntime, plugin::{Plugin, PluginContext}, render::{RenderExtraction, RenderRuntime}, runtime::RuntimeError, simulation::{Simulation, SimulationRuntime, SimulationTask}, window::WindowManager};

pub struct Engine<R: Renderer, G: Game<Plugins: Plugin>> {
    input: InputRuntime,
    main: MainRuntime,
    render: RenderRuntime<R>,
    simulation: SimulationRuntime,
    game: G,
    windows: WindowManager<R::Surface>,
}

impl<R: Renderer, G: Game<Plugins: Plugin>> Engine<R, G> {
    pub fn new(renderer: R, game: G) -> Self {
        Self {
            input: InputRuntime::new(),
            main: MainRuntime::new(),
            render: RenderRuntime::new(renderer),
            simulation: SimulationRuntime::new(),
            game,
            windows: WindowManager::new(),
        }
    }

    fn start<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), RuntimeError> {
        let mut render_extraction = RenderExtraction::new();
        let mut simulation = Simulation::new();
        let mut plugins = self.game.plugins();
        let (main_registry, main_schedule) = self.main.parts_mut();

        let mut context = PluginContext::new(
            G::METADATA,
            main_registry,
            main_schedule,
            self.windows.plan_mut(),
            self.render.plan_mut(),
            &mut render_extraction,
            &mut simulation,
            self.simulation.plan_mut(),
        );

        plugins.build(&mut context)?;
        drop(context);

        self.windows.apply(self.main.registry_mut(), self.render.renderer_mut(), handle)?;

        let render_scene_producer = self.render.start();
        let simulation_task = SimulationTask::new(simulation, render_extraction, render_scene_producer);

        self.simulation.start(simulation_task)?;

        Ok(())
    }

    fn tick(&mut self) -> Result<(), RuntimeError> {
        self.main.update(self.input.state());
        self.simulation.update()?;
        self.render.update(self.main.registry())?;
        self.windows.request_redraws();

        Ok(())
    }

    fn stop<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), RuntimeError> {
        let simulation_error = self.simulation.stop();

        self.render.stop();
        self.windows.destroy_all(self.main.registry_mut(), handle)?;

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
        self.main.resume();
        self.windows.resume_surfaces(self.render.renderer_mut())?;
        self.simulation.resume();

        Ok(())
    }

    fn suspended<H: BackendHandle>(&mut self, _handle: &mut H) -> Result<(), Self::Error> {
        self.input.reset();
        self.main.suspend();
        self.simulation.suspend();
        self.render.suspend();
        self.windows.suspend_surfaces();

        Ok(())
    }

    fn update<H: BackendHandle>(&mut self, _handle: &mut H) -> Result<(), Self::Error> {
        let result = self.tick();
        self.input.clear_transient();

        result
    }

    fn input_event(&mut self, event: InputEvent) {
        self.input.update(event);
    }

    fn window_event<H: BackendHandle>(&mut self, handle: &mut H, event: WindowEvent)
        -> Result<(), Self::Error>
    {
        if let WindowEventKind::CloseRequested = event.kind {
            return self.windows.close_requested(self.main.registry_mut(), handle, event.window_id);
        }

        self.windows.synchronize(self.main.registry_mut(), event);

        if let WindowEventKind::Focused { focused: false } = event.kind {
            self.input.release_all();
        }

        if let WindowEventKind::Resized { width, height } = event.kind {
            if let Some(surface) = self.windows.surface_mut(event.window_id) {
                self.render.resize_surface(surface, (width, height))?;
            }
        }

        if let WindowEventKind::RedrawRequested = event.kind {
            if let Some((target, surface)) = self.windows.render_target_mut(event.window_id) {
                self.render.render(surface, target)?;
            }
        }

        Ok(())
    }

    fn stopped<H: BackendHandle>(&mut self, handle: &mut H) -> Result<(), Self::Error> {
        self.stop(handle)
    }
}
