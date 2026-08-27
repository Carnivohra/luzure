use luzure_backend::window::WindowDescriptor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowState {
    title: String,
    width: u32,
    height: u32,
    resizable: bool,
    visible: bool,
    focused: bool,
}

impl WindowState {
    pub(crate) fn new(descriptor: WindowDescriptor, width: u32, height: u32) -> Self {
        Self {
            title: descriptor.title,
            width,
            height,
            resizable: descriptor.resizable,
            visible: descriptor.visible,
            focused: false,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub const fn width(&self) -> u32 {
        self.width
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub const fn resizable(&self) -> bool {
        self.resizable
    }

    pub const fn visible(&self) -> bool {
        self.visible
    }

    pub const fn focused(&self) -> bool {
        self.focused
    }

    pub(super) fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub(super) fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub(super) fn set_title(&mut self, title: &str) {
        self.title.clear();
        self.title.push_str(title);
    }
}
