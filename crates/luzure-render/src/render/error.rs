use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("failed to find a compatible graphics adapter")]
    AdapterRequest,

    #[error("failed to create graphics device")]
    DeviceRequest,

    #[error("render surface dimensions must be nonzero and within device limits")]
    InvalidSurfaceSize,

    #[error("render mesh must contain vertices and valid triangle indices")]
    InvalidMesh,

    #[error("render mesh handle is invalid")]
    InvalidMeshHandle,

    #[error("render mesh index count exceeds supported capacity")]
    MeshCapacityExceeded,

    #[error("render instance data exceeds supported capacity")]
    InstanceCapacityExceeded,

    #[error("render view data exceeds supported capacity")]
    ViewCapacityExceeded,

    #[error("render pipeline is not available for the surface format")]
    PipelineUnavailable,

    #[error("failed to acquire render surface texture")]
    SurfaceAcquisition,

    #[error("failed to create render surface")]
    SurfaceCreation,

    #[error("render surface is not supported by the graphics adapter")]
    SurfaceUnsupported,
}
