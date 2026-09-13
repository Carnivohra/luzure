mod application;
mod handle;
mod run;

use handle::WinitBackendHandle;
use run::run;

use luzure_backend::{Backend, backend::BackendApplication};

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(not(target_os = "android"))]
pub struct WinitBackend;

#[cfg(target_os = "android")]
pub struct WinitBackend {
    android_app: AndroidApp,
}

#[cfg(not(target_os = "android"))]
impl WinitBackend {
    pub const fn new() -> Self {
        Self
    }
}

#[cfg(target_os = "android")]
impl WinitBackend {
    pub fn new(android_app: AndroidApp) -> Self {
        Self {
            android_app,
        }
    }
}

#[cfg(not(target_os = "android"))]
impl Backend for WinitBackend {
    fn run<A: BackendApplication + 'static>(self, application: A) -> Result<(), A::Error> {
        run(application)
    }
}

#[cfg(target_os = "android")]
impl Backend for WinitBackend {
    fn run<A: BackendApplication + 'static>(self, application: A) -> Result<(), A::Error> {
        run(self.android_app, application)
    }
}
