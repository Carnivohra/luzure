mod descriptor;
mod event;
mod id;
mod source;

pub use descriptor::WindowDescriptor;
pub use event::WindowEvent;
pub use id::WindowId;
use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle};
pub use source::WindowSource;

pub struct Window {
    id: WindowId,
    source: Box<dyn WindowSource>,
}

impl Window {
    pub fn new<S: WindowSource + 'static>(id: WindowId, source: Box<S>) -> Self {
        Self {
            id,
            source,
        }
    }

    pub const fn id(&self) -> WindowId {
        self.id
    }

    pub fn inner_size(&self) -> (u32, u32) {
        self.source.inner_size()
    }

    pub fn set_title(&self, title: &str) {
        self.source.set_title(title);
    }

    pub fn request_redraw(&self) {
        self.source.request_redraw();
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.source.display_handle()
    }
}

impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        self.source.window_handle()
    }
}
