use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub trait WindowSource: HasDisplayHandle + HasWindowHandle + Send + Sync {
    fn inner_size(&self) -> (u32, u32);
    fn set_title(&self, title: &str);
    fn request_redraw(&self);
}
