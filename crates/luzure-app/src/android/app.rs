use luzure_game::Game;
use luzure_runtime::{Plugin, runtime::RuntimeError};
use luzure_wgpu::WgpuRenderer;
use luzure_winit::WinitBackend;

use crate::{AndroidAppHandle, App};

pub struct AndroidApp {
    app: App<WinitBackend, WgpuRenderer>,
}

impl AndroidApp {
    pub fn new(android_app: AndroidAppHandle) -> Self {
        Self { app: App::new(WinitBackend::new(android_app), WgpuRenderer::new()) }
    }

    pub fn run<G: Game<Plugins: Plugin> + 'static>(self, game: G) -> Result<(), RuntimeError> {
        self.app.run(game)
    }
}
