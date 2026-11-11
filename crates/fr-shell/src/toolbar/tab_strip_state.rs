use fr_icons::catalog::icon::Icon;
use fr_ui::hover::chrome_hover::ChromeHover;

pub struct TabStripState {
    pub titles: Vec<String>,
    pub icons: Vec<Icon>,
    pub discarded: Vec<bool>,
    pub active: usize,
    pub hover: ChromeHover,
    pub maximized: bool,
}
