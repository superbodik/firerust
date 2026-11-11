use crate::tabs::window_control::WindowControl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabStripTarget {
    Tab(usize),
    Close(usize),
    New,
    Control(WindowControl),
    Empty,
}
