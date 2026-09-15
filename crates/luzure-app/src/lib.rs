mod app;

#[cfg(all(feature = "wgpu", feature = "winit", target_os = "android"))]
mod android;

#[cfg(all(feature = "wgpu", feature = "winit", not(target_family = "wasm"), not(target_os = "android"), not(target_os = "ios")))]
mod desktop;

#[cfg(all(feature = "wgpu", feature = "winit", target_os = "ios"))]
mod ios;

#[cfg(all(feature = "wgpu", feature = "winit", target_family = "wasm"))]
mod web;

pub use app::App;

#[cfg(all(feature = "wgpu", feature = "winit", target_os = "android"))]
pub use android::{AndroidApp, AndroidAppHandle};

#[cfg(all(feature = "wgpu", feature = "winit", not(target_family = "wasm"), not(target_os = "android"), not(target_os = "ios")))]
pub use desktop::DesktopApp;

#[cfg(all(feature = "wgpu", feature = "winit", target_os = "ios"))]
pub use ios::IosApp;

#[cfg(all(feature = "wgpu", feature = "winit", target_family = "wasm"))]
pub use web::WebApp;
