use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("failed to find a compatible graphics adapter")]
    AdapterRequest,

    #[error("failed to create graphics device")]
    DeviceRequest,

    #[error("render surface dimensions must be greater than zero")]
    InvalidSurfaceSize,
    
    #[error("failed to acquire render surface texture")]
    SurfaceAcquisition,

    #[error("failed to create render surface")]
    SurfaceCreation,

    #[error("render surface is not supported by the graphics adapter")]
    SurfaceUnsupported,
}
