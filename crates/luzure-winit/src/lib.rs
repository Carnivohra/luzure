mod backend;
mod input;
mod window;

pub use backend::WinitBackend;

#[cfg(target_os = "android")]
pub use winit::platform::android::activity::AndroidApp;
