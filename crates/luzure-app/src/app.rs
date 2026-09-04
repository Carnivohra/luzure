use luzure_backend::Backend;
use luzure_game::Game;
use luzure_render::Renderer;
use luzure_runtime::{Engine, runtime::RuntimeError};

pub struct App<B: Backend, R: Renderer> {
    backend: B,
    renderer: R,
}

impl<B: Backend, R: Renderer> App<B, R> {
    pub fn new(backend: B, renderer: R) -> Self {
        Self {
            backend,
            renderer,
        }
    }

    pub fn run<G: Game>(self, game: G) -> Result<(), RuntimeError> {
        self.backend.run(Engine::new(self.renderer, game))
    }
}
