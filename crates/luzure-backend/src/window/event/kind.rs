#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowEventKind {
    CloseRequested,
    RedrawRequested,
    Resized { width: u32, height: u32 },
    Focused { focused: bool },
}
