use luzure_backend::backend::{BackendApplication, BackendError};
use winit::{event_loop::EventLoop, platform::android::{EventLoopBuilderExtAndroid, activity::AndroidApp}};

use crate::backend::application::WinitApplication;

pub(in crate::backend) fn run<A: BackendApplication + 'static>(android_app: AndroidApp, application: A)
    -> Result<(), A::Error>
{
    let event_loop = EventLoop::builder()
        .with_android_app(android_app)
        .build()
        .map_err(|_| BackendError::EventLoopInitialization)?;

    let mut application = WinitApplication::new(application);

    event_loop.run_app(&mut application)
        .map_err(|_| BackendError::EventLoop)?;

    if let Some(error) = application.take_error() {
        return Err(error);
    }

    Ok(())
}
