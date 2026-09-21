#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderSceneTransfer {
    Automatic,
    Direct,
    TripleBuffered,
}
