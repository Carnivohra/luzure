use luzure_backend::backend::BackendError;
use luzure_render::render::RenderError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error(transparent)]
    Backend(#[from] BackendError),

    #[error(transparent)]
    Render(#[from] RenderError),
}
