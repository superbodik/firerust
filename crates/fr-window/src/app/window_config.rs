use crate::app::window_icon::WindowIcon;

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub icon: Option<WindowIcon>,
}

impl WindowConfig {
    pub fn new(title: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            title: title.into(),
            width,
            height,
            icon: None,
        }
    }

    pub fn with_icon(mut self, icon: Option<WindowIcon>) -> Self {
        self.icon = icon;
        self
    }
}
