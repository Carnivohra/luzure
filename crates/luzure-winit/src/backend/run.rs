#[cfg(target_os = "android")]
mod android;

#[cfg(all(not(target_family = "wasm"), not(target_os = "android"), not(target_os = "ios")))]
mod desktop;

#[cfg(target_os = "ios")]
mod ios;

#[cfg(not(target_family = "wasm"))]
mod native;

#[cfg(target_family = "wasm")]
mod web;

#[cfg(target_os = "android")]
pub(super) use android::run;

#[cfg(all(not(target_family = "wasm"), not(target_os = "android"), not(target_os = "ios")))]
pub(super) use desktop::run;

#[cfg(target_os = "ios")]
pub(super) use ios::run;

#[cfg(target_family = "wasm")]
pub(super) use web::run;
