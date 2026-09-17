#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RendererStatus {
    Uninitialized,
    Initializing,
    Ready,
}
