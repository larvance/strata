//! Strata Platform: Native windowing and OS input abstraction.

pub enum PlatformTarget {
    Android,
    Ios,
    MacOs,
    Windows,
    Linux,
    WebWasm,
    WebJs,
}

pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Strata Application".into(),
            width: 1280,
            height: 800,
            resizable: true,
        }
    }
}

pub trait PlatformWindow {
    fn set_title(&mut self, title: &str);
    fn request_redraw(&mut self);
}
