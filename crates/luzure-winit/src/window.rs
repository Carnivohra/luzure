mod entry;

pub(crate) use entry::WinitWindowEntry;

use luzure_backend::window::WindowSource;
use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle};
use winit::window::Window;

pub(crate) struct WinitWindow {
    window: Window,
}

impl WinitWindow {
    pub(crate) const fn new(window: Window) -> Self {
        Self { window }
    }
}

impl WindowSource for WinitWindow {
    fn inner_size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }

    fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    fn request_redraw(&self) {
        self.window.request_redraw();
    }
}

impl HasDisplayHandle for WinitWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.window.display_handle()
    }
}

impl HasWindowHandle for WinitWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        self.window.window_handle()
    }
}
