use luzure_backend::backend::{BackendApplication, BackendError};
use winit::{event_loop::EventLoop, platform::android::{EventLoopBuilderExtAndroid, activity::AndroidApp}};

pub(in crate::backend) fn run<A: BackendApplication + 'static>(android_app: AndroidApp, application: A)
    -> Result<(), A::Error>
{
    let event_loop = EventLoop::builder()
        .with_android_app(android_app)
        .build()
        .map_err(|_| BackendError::EventLoopInitialization)?;

    super::native::run(event_loop, application)
}
