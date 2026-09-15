use luzure_game::Game;
use luzure_runtime::{Plugin, runtime::RuntimeError};
use luzure_wgpu::WgpuRenderer;
use luzure_winit::WinitBackend;

use crate::App;

pub struct IosApp {
    app: App<WinitBackend, WgpuRenderer>,
}

impl IosApp {
    pub fn new() -> Self {
        Self { app: App::new(WinitBackend::new(), WgpuRenderer::new()) }
    }

    pub fn run<G: Game<Plugins: Plugin> + 'static>(self, game: G) -> Result<(), RuntimeError> {
        self.app.run(game)
    }
}
